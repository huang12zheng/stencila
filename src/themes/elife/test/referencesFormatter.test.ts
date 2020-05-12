import * as referencesFormatter from '../lib/referencesFormatter'
import { getFixtureData } from './fixtures/referencesData.fixture'
import { select, translate } from '../../../util'

const getFirst = (references: Element[]): Element => {
  const firstReference = references[0]
  if (firstReference === null || firstReference === undefined) {
    throw new Error('No reference found in fixture data')
  }
  return firstReference
}

const getElement = (searchRoot: Element, customSelector: string): Element => {
  const element = searchRoot.querySelector(translate(customSelector))
  if (element === null) {
    throw new Error(
      `No elements found matching the semantic selector ${customSelector}`
    )
  }
  return element
}

describe('Formatting a reference', () => {
  let references: Element[]
  let firstReference: Element

  beforeEach(() => {
    references = select(getFixtureData(), ':--reference')
    firstReference = getFirst(references)
  })

  it('the title is the first element', () => {
    referencesFormatter.format(references)
    expect(
      getElement(firstReference, ':--title').isSameNode(
        firstReference.firstElementChild
      )
    ).toBe(true)
  })

  it('the authors follow the title', () => {
    referencesFormatter.format(references)

    expect(
      getElement(firstReference, ':--authors').isSameNode(
        firstReference.children[1]
      )
    ).toBe(true)
  })
  it('the publication year follows the authors', () => {
    referencesFormatter.format(references)

    expect(
      getElement(firstReference, ':--datePublished').isSameNode(
        firstReference.children[2]
      )
    ).toBe(true)
  })

  it('the publication name follows the publication year', () => {
    referencesFormatter.format(references)
    const volume = getElement(firstReference, ':--PublicationVolume')
    expect(volume.isSameNode(firstReference.children[3])).toBe(true)

    const volumePart = getElement(volume, ':--isPartOf')
    expect(volumePart.isSameNode(volume.children[0])).toBe(true)
    expect(getElement(volumePart, ':--name').isSameNode(volumePart.children[0]))
  })
})
