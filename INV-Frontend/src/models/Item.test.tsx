import { describe, it, expect } from 'vitest';
import { Item, ItemDto } from './Item';
import { NO_DATA_MESSAGE } from '../constants';

describe('Item Class', () => {
  describe('Item with valid data fields', () => {
    const expectedItemId = 1;
    const expectedItemName = 'Item A';
    const expectedNbOfItems = 10;
    const input: ItemDto = {
      item_id: expectedItemId,
      item_name: expectedItemName,
      nb_of_items: expectedNbOfItems,
    };
    const item = new Item(input);

    it('should initialize with expected item id', () => {
      expect(item.itemId).toBe(expectedItemId);
    });

    it('should initialize with expected item name', () => {
      expect(item.itemName).toBe(expectedItemName);
    });

    it('should initialize with expected number of items', () => {
      expect(item.nbOfItems).toBe(expectedNbOfItems);
    });
  });

  describe('Item with zero items', () => {
    const expectedItemId = 1;
    const expectedItemName = 'Item A';
    const expectedNbOfItems = 0;
    const input: ItemDto = {
      item_id: expectedItemId,
      item_name: expectedItemName,
      nb_of_items: expectedNbOfItems,
    };
    const item = new Item(input);

    it('should initialize with expected number of items', () => {
      expect(item.nbOfItems).toBe(expectedNbOfItems);
    });
  });

  describe('Item with negative items', () => {
    const expectedItemId = 1;
    const expectedItemName = 'Item A';
    const expectedNbOfItems = -1;
    const expectedValue = NO_DATA_MESSAGE;
    const input: ItemDto = {
      item_id: expectedItemId,
      item_name: expectedItemName,
      nb_of_items: expectedNbOfItems,
    };
    const item = new Item(input);

    it('should initialize with expected NO_DATA_MESSAGE for number of items', () => {
      expect(item.nbOfItems).toBe(expectedValue);
    });
  });

  describe('Item with undefined fields', () => {
    const expectedValue = NO_DATA_MESSAGE;
    const input: ItemDto = {
      item_id: undefined,
      item_name: undefined,
      nb_of_items: undefined,
    };
    const item = new Item(input);

    it('should initialize with expected NO_DATA_MESSAGE for item id', () => {
      expect(item.itemId).toBe(expectedValue);
    });

    it('should initialize with expected NO_DATA_MESSAGE for item name', () => {
      expect(item.itemName).toBe(expectedValue);
    });

    it('should initialize with expected NO_DATA_MESSAGE for number of items', () => {
      expect(item.nbOfItems).toBe(expectedValue);
    });
  });
});
