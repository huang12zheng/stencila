import { applyAdd, applyAddOption, applyAddText, applyAddVec } from './add'

test('applyAddOption', () => {
  const elem = document.createElement('div')
  const html = '<p slot="property">A fragment</p>'

  applyAddOption(elem, 'property', html)
  expect(elem.querySelector('[slot="property"]')?.outerHTML).toEqual(html)

  expect(() => applyAddOption(elem, 1, '')).toThrow(/Expected string slot/)
})

test('applyAddVec', () => {
  const elem = document.createElement('div')

  applyAddVec(elem, 0, '<p>one</p>')
  expect(elem.innerHTML).toEqual('<p>one</p>')

  applyAddVec(elem, 1, '<p>two</p>')
  expect(elem.innerHTML).toEqual('<p>one</p><p>two</p>')

  applyAddVec(elem, 0, '<p>zero</p>')
  expect(elem.innerHTML).toEqual('<p>zero</p><p>one</p><p>two</p>')

  applyAddVec(elem, 2, '<p>1.5</p>')
  expect(elem.innerHTML).toEqual('<p>zero</p><p>one</p><p>1.5</p><p>two</p>')

  applyAddVec(elem, 4, '<p>three</p>')
  expect(elem.innerHTML).toEqual(
    '<p>zero</p><p>one</p><p>1.5</p><p>two</p><p>three</p>'
  )

  expect(() => applyAddVec(elem, 'string', '')).toThrow(/Expected number slot/)
  expect(() => applyAddVec(elem, -1, '')).toThrow(/Unexpected add slot '-1'/)
  expect(() => applyAddVec(elem, 42, '')).toThrow(/Unexpected add slot '42'/)
})

test('applyAddText', () => {
  const node = document.createTextNode('')

  applyAddText(node, 0, 'a')
  expect(node.textContent).toEqual('a')

  applyAddText(node, 1, 'e')
  expect(node.textContent).toEqual('ae')

  applyAddText(node, 1, 'bcd')
  expect(node.textContent).toEqual('abcde')

  applyAddText(node, 2, '🏳️‍🌈')
  expect(node.textContent).toEqual('ab🏳️‍🌈cde')

  applyAddText(node, 4, '🎁')
  expect(node.textContent).toEqual('ab🏳️‍🌈c🎁de')

  expect(() => applyAddText(node, 'string', '')).toThrow(/Expected number slot/)
  expect(() => applyAddText(node, -1, '')).toThrow(/Unexpected add slot '-1'/)
  expect(() => applyAddText(node, 42, '')).toThrow(/Unexpected add slot '42'/)
})

test('applyAdd', () => {
  // Start with an empty `Article`
  document.body.innerHTML = '<article data-itemscope="root"></article>'
  expect(document.body).toMatchInlineSnapshot(`
    <body>
      <article
        data-itemscope="root"
      />
    </body>
  `)

  // Add a empty `Paragraph` to the `Article`'s optional `content` property
  applyAdd({
    type: 'Add',
    address: ['content'],
    html: `<div data-itemprop="content"><p></p></div>`,
    json: {},
  })
  expect(document.body).toMatchInlineSnapshot(`
    <body>
      <article
        data-itemscope="root"
      >
        <div
          data-itemprop="content"
        >
          <p />
        </div>
      </article>
    </body>
  `)

  // Add a `String` node to the `Paragraph`'s implicit `content` property
  applyAdd({
    type: 'Add',
    address: ['content', 0, 'content', 0],
    html: 'Some text.',
    json: {},
  })
  expect(document.body).toMatchInlineSnapshot(`
<body>
  <article
    data-itemscope="root"
  >
    <div
      data-itemprop="content"
    >
      <p>
        Some text.
      </p>
    </div>
  </article>
</body>
`)

  // Insert some characters ito the `String` node
  applyAdd({
    type: 'Add',
    address: ['content', 0, 'content', 0, 5],
    html: 'more ',
    json: {},
  })
  expect(document.body).toMatchInlineSnapshot(`
<body>
  <article
    data-itemscope="root"
  >
    <div
      data-itemprop="content"
    >
      <p>
        Some more text.
      </p>
    </div>
  </article>
</body>
`)

  // Insert some inline content before the existing `String`
  applyAdd({
    type: 'Add',
    address: ['content', 0, 'content', 0],
    html: 'Some <strong>strong</strong> text. ',
    json: {},
  })
  expect(document.body).toMatchInlineSnapshot(`
<body>
  <article
    data-itemscope="root"
  >
    <div
      data-itemprop="content"
    >
      <p>
        Some 
        <strong>
          strong
        </strong>
         text. 
        Some more text.
      </p>
    </div>
  </article>
</body>
`)
})