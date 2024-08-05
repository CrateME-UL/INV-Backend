import { NO_DATA_MESSAGE } from '../constants';
import {
  translatePlaceTypeFR,
  getPlaceTypeColor,
} from '../utils/placeTypeUtils';

export type InventoryPlaceDto = {
  place_id: number | undefined;
  place_name: string | undefined;
  place_type: string | undefined;
  nb_of_items: number | undefined;
};

export const buildInventoryPlace = (
  place: InventoryPlaceDto
): InventoryPlace => {
  return new InventoryPlace(place);
};

export class InventoryPlace {
  placeId: number | string;
  placeName: string;
  placeType: string;
  nbOfItems: number | string;
  placeTypeFrench: string;
  placeTypeColor: string;

  constructor(place: InventoryPlaceDto) {
    this.placeId = place.place_id || NO_DATA_MESSAGE;
    this.placeName = place.place_name || NO_DATA_MESSAGE;
    this.placeType = place.place_type || NO_DATA_MESSAGE;
    this.nbOfItems =
      place.nb_of_items !== undefined
        ? place.nb_of_items >= 0
          ? place.nb_of_items
          : NO_DATA_MESSAGE
        : NO_DATA_MESSAGE;
    this.placeTypeFrench = translatePlaceTypeFR(this.placeType);
    this.placeTypeColor = getPlaceTypeColor(this.placeType);
  }
}
