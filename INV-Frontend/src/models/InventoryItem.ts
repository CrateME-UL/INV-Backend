import { NO_DATA_MESSAGE } from '../constants';

export type InventoryItemDto = {
  item_id: number | undefined;
  item_name: string | undefined;
  nb_of_items: number | undefined;
};

export const buildInventoryItem = (
  item: InventoryItemDto
): InventoryItem => {
  return new InventoryItem(item);
};

export class InventoryItem {
  itemId: number | string;
  itemName: string;
  nbOfItems: number | string;

  constructor(item: InventoryItemDto) {
    this.itemId = item.item_id || NO_DATA_MESSAGE;
    this.itemName = item.item_name || NO_DATA_MESSAGE;
    this.nbOfItems =
      item.nb_of_items !== undefined
        ? item.nb_of_items >= 0
          ? item.nb_of_items
          : NO_DATA_MESSAGE
        : NO_DATA_MESSAGE;
  }
}
