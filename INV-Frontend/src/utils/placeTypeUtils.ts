import {
  DEFAULT_COLOR,
  IN_COLOR,
  INV_COLOR,
  NO_DATA_MESSAGE,
  OUT_COLOR,
} from '../constants';

export const translatePlaceTypeFR = (placeType: string): string => {
  switch (placeType) {
    case 'INV':
      return 'INV';
    case 'OUT':
      return 'EXT';
    case 'IN':
      return 'INT';
    default:
      return NO_DATA_MESSAGE;
  }
};

export const translatePlaceTypeEN = (placeType: string): string => {
  switch (placeType) {
    case 'INV':
      return 'INV';
    case 'EXT':
      return 'OUT';
    case 'INT':
      return 'IN';
    default:
      return NO_DATA_MESSAGE;
  }
};

export const getPlaceTypeColor = (placeType: string): string => {
  switch (placeType) {
    case 'INV':
      return INV_COLOR;
    case 'EXT':
      return OUT_COLOR;
    case 'INT':
      return IN_COLOR;
    default:
      return DEFAULT_COLOR;
  }
};
