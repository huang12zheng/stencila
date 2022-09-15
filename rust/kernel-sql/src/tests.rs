use kernel::{
    common::{itertools::Itertools, tokio},
    stencila_schema::{
        ArrayValidator, BooleanValidator, DatatableColumn, Date, DateTime, DateTimeValidator,
        DateValidator, IntegerValidator, Number, NumberValidator, Primitive, StringValidator, Time,
        TimeValidator, ValidatorTypes,
    },
    KernelTrait,
};
use test_utils::{assert_json_eq, assert_json_is, skip_ci};

use super::*;

/// Test against SQLite
#[tokio::test]
async fn test_sqlite() -> Result<()> {
    test("sqlite://:memory:").await
}

/// Test against DuckDB
#[tokio::test]
async fn test_duckdb() -> Result<()> {
    test("duckdb://:memory:").await
}

/// Test against Postgres
///
/// Requires some manual setup:
///   > docker run --rm -it -p5432:5432 -e POSTGRES_PASSWORD=postgres postgres
///   > PGPASSWORD=postgres createdb --host=localhost --user postgres testdb
#[tokio::test]
async fn test_postgres() -> Result<()> {
    if skip_ci("Not yet setup to work on CI") {
        return Ok(());
    };
    test("postgres://postgres:postgres@localhost:5432/testdb").await
}

/// General integration test
async fn test(config: &str) -> Result<()> {
    let mut kernel = SqlKernel::new(
        &KernelSelector {
            config: Some(config.to_string()),
            ..Default::default()
        },
        None,
    );

    // Clean up after any previous test
    kernel
        .exec("DROP TABLE IF EXISTS table_a", Format::SQL, None)
        .await?;

    // Test that getting a non-existent table does not work
    if let Ok(..) = kernel.get("table_a").await {
        bail!("Expected an error because table not yet created")
    };

    // Test setting a Datatable
    // Always use `items_nullable: true` because, for this test, that is what we get
    // in the Datatables we get back from the database
    let rows = 5;
    let col_bool = DatatableColumn {
        name: "col_bool".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::BooleanValidator(
                BooleanValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: vec![Primitive::Boolean(true); rows],
        ..Default::default()
    };
    let col_int = DatatableColumn {
        name: "col_int".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::IntegerValidator(
                IntegerValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: (0..rows)
            .map(|index| Primitive::Integer(index as i64))
            .collect_vec(),
        ..Default::default()
    };
    let col_num = DatatableColumn {
        name: "col_num".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::NumberValidator(
                NumberValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: (0..rows)
            .map(|index| Primitive::Number(Number(index as f64)))
            .collect_vec(),
        ..Default::default()
    };
    let col_str = DatatableColumn {
        name: "col_str".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::StringValidator(
                StringValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: (0..rows)
            .map(|index| Primitive::String(format!("string-{}", index)))
            .collect_vec(),
        ..Default::default()
    };
    let col_date = DatatableColumn {
        name: "col_date".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::DateValidator(
                DateValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: (0..rows)
            .map(|index| Primitive::Date(Date::from(format!("2000-01-0{}", index + 1))))
            .collect_vec(),
        ..Default::default()
    };
    let col_time = DatatableColumn {
        name: "col_time".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::TimeValidator(
                TimeValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: (0..rows)
            .map(|index| Primitive::Time(Time::from(format!("00:00:0{}", index))))
            .collect_vec(),
        ..Default::default()
    };
    let col_datetime = DatatableColumn {
        name: "col_datetime".to_string(),
        validator: Some(Box::new(ArrayValidator {
            items_validator: Some(Box::new(ValidatorTypes::DateTimeValidator(
                DateTimeValidator::default(),
            ))),
            items_nullable: true,
            ..Default::default()
        })),
        values: (0..rows)
            .map(|index| {
                Primitive::DateTime(DateTime::from(format!(
                    "2000-01-0{}T00:00:0{}",
                    index + 1,
                    index + 1
                )))
            })
            .collect_vec(),
        ..Default::default()
    };

    let datatable_a = Datatable {
        columns: vec![
            col_bool,
            col_int,
            col_num,
            col_str,
            col_date,
            col_time,
            col_datetime,
        ],
        ..Default::default()
    };

    kernel
        .set("table_a", Node::Datatable(datatable_a.clone()))
        .await?;
    let table_a = kernel.get("table_a").await?;
    assert_json_eq!(table_a, datatable_a);

    // Test that @assign tag works as expected
    kernel
        .exec(
            "SELECT * FROM table_a",
            Format::SQL,
            Some(&TagMap::from_name_values(&[("assigns", "query_1")])),
        )
        .await?;
    let query_1 = kernel.get("query_1").await?;
    assert_json_eq!(query_1, datatable_a);

    // Test that possibly untyped columns (at least in SQLite) are translated into values
    let query_2 = kernel.exec("SELECT 123;", Format::SQL, None).await?;
    match &query_2.0[0] {
        Node::Datatable(datatable) => {
            assert_eq!(datatable.columns[0].values[0], Primitive::Integer(123))
        }
        _ => bail!("Should be a datatable!"),
    }

    // Test setting a non-Datatable adds to the kernel's parameters which can then be used in bindings
    kernel.set("param", Node::Integer(3)).await?;
    kernel.parameters.contains_key("param");
    let query_3 = kernel
        .exec(
            "SELECT col_str FROM table_a WHERE col_int = $param;",
            Format::SQL,
            None,
        )
        .await?;
    match &query_3.0[0] {
        Node::Datatable(datatable) => {
            assert_eq!(
                datatable.columns[0].values[0],
                Primitive::String("string-3".to_string())
            )
        }
        _ => bail!("Should be a datatable!"),
    }

    // Test deriving parameters from columns

    kernel
        .exec(
            r#"
                CREATE TABLE table_1 (
                    col_a BOOLEAN,
                    col_b INTEGER DEFAULT 42,
                    col_c DATE CHECK (col_c > '2001-01-01') DEFAULT '2001-01-02'
                )"#,
            Format::SQL,
            None,
        )
        .await?;

    let parameter = kernel.derive("parameter", "table_1.col_a").await?;
    assert_json_is!(parameter, [{
        "type": "Parameter",
        "name": "col_a",
        "validator": {
            "type": "BooleanValidator"
        }
    }]);

    let parameter = kernel.derive("parameter", "table_1.col_b").await?;
    assert_json_is!(parameter, [{
        "type": "Parameter",
        "name": "col_b",
        "validator": {
            "type": "IntegerValidator"
        },
        "default": 42
    }]);

    let parameter = kernel.derive("parameter", "table_1.col_c").await?;
    if config.starts_with("duckdb://") {
        // DuckDB does not yet support retrieval of checks
        assert_json_is!(parameter, [{
            "type": "Parameter",
            "name": "col_c",
            "validator": {
                "type": "DateValidator",
            },
            "default": {
                "type": "Date",
                "value": "2001-01-02"
            }
        }]);
    } else {
        assert_json_is!(parameter, [{
            "type": "Parameter",
            "name": "col_c",
            "validator": {
                "type": "DateValidator",
                "minimum": {
                    "type": "Date",
                    "value": "2001-01-01"
                }
            },
            "default": {
                "type": "Date",
                "value": "2001-01-02"
            }
        }]);
    }

    let parameters = kernel.derive("parameters", "table_1").await?;
    assert_eq!(parameters.len(), 3);

    if let Err(error) = kernel.derive("parameter", "table_1.col_foo").await {
        assert!(
            error
                .to_string()
                .starts_with("Column `col_foo` does not appear to exist"),
            "Got `{}`",
            error
        )
    } else {
        bail!("Expected error")
    }

    if let Err(error) = kernel.derive("parameter", "table_bar.col_foo").await {
        assert!(
            error
                .to_string()
                .starts_with("Table `table_bar` does not appear to exist"),
            "Got `{}`",
            error
        )
    } else {
        bail!("Expected error")
    }

    if config.starts_with("sqlite://") {
        kernel
            .exec(
                r#"
                CREATE TABLE table_2 (
                    col_enum TEXT CHECK(col_enum IN ('one', 'two', 'three')) DEFAULT 'two'
                )"#,
                Format::SQL,
                None,
            )
            .await?;
    } else {
        kernel
            .exec(
                r#"
                CREATE TYPE my_enum AS ENUM ('one', 'two', 'three');
                CREATE TABLE table_2 (
                    col_enum my_enum DEFAULT 'two'
                )"#,
                Format::SQL,
                None,
            )
            .await?;
    }

    let parameter = kernel.derive("parameter", "table_2.col_enum").await?;
    assert_json_is!(parameter, [{
        "type": "Parameter",
        "name": "col_enum",
        "default": "two",
        "validator": {
            "type": "EnumValidator",
            "values": ["one", "two", "three"]
        }
    }]);

    Ok(())
}
