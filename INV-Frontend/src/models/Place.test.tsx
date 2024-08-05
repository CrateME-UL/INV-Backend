import { describe, it, expect } from 'vitest';
import { Place, PlaceDto } from './Place';
import { NO_DATA_MESSAGE } from '../constants';

describe('Place Class', () => {
  describe('Place with valid data fields', () => {
    const expectedPlaceId = 1;
    const expectedPlaceName = 'Place A';
    const expectedPlaceType = 'INV';
    const input: PlaceDto = {
      place_id: expectedPlaceId,
      place_name: expectedPlaceName,
      place_type: expectedPlaceType,
    };
    const place = new Place(input);

    it('should initialize with expected place id', () => {
      expect(place.placeId).toBe(expectedPlaceId);
    });

    it('should initialize with expected place name', () => {
      expect(place.placeName).toBe(expectedPlaceName);
    });

    it('should initialize with expected place type', () => {
      expect(place.placeType).toBe(expectedPlaceType);
    });
  });

  describe('Place with undefined fields', () => {
    const expectedValue = NO_DATA_MESSAGE;
    const input: PlaceDto = {
      place_id: undefined,
      place_name: undefined,
      place_type: undefined,
    };
    const place = new Place(input);
    it('should initialize with expected NO_DATA_MESSAGE for place id', () => {
      expect(place.placeId).toBe(expectedValue);
    });

    it('should initialize with expected NO_DATA_MESSAGE for place name', () => {
      expect(place.placeName).toBe(expectedValue);
    });

    it('should initialize with expected NO_DATA_MESSAGE for place type', () => {
      expect(place.placeType).toBe(expectedValue);
    });
  });
});
