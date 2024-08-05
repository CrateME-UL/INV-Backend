import { NO_DATA_MESSAGE } from '../constants';

export type ItemDto = {
  item_id: number | undefined;
  item_name: string | undefined;
  nb_of_items: number | undefined;
};

export const buildItem = (item: ItemDto): Item => {
  return new Item(item);
};

export class Item {
  itemId: number | string;
  itemName: string;
  nbOfItems: number | string;

  constructor(item: ItemDto) {
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
