import * as React from 'react';
import { getResponse } from '../API';
import { CustomDataGrid } from '../components/CustomDataGrid';
import { GridColDef } from '@mui/x-data-grid';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import { buildItem, Item, ItemDto } from '../models/Item';
import { SelectFilter } from '../components/SelectFilter';
import {
  buildInventoryPlace,
  InventoryPlace,
  InventoryPlaceDto,
} from '../models/InventoryPlace';
import Chip from '@mui/material/Chip';
import ChipFilter from '../components/ChipFilter';
import { ALL_PLACE_TYPES_FR, CHIP_PROPERTIES } from '../constants';
import {
  getPlaceTypeColor,
  translatePlaceTypeEN,
  translatePlaceTypeFR,
} from '../utils/placeTypeUtils';

interface InventoryState {
  places: InventoryPlace[];
  items: Item[];
  error: string | null;
  itemsFilter: string;
}

type InventoryAction =
  | { type: 'SET_PLACES'; payload: InventoryPlace[] }
  | { type: 'SET_ITEMS'; payload: Item[] }
  | { type: 'SET_ERROR'; payload: string | null }
  | { type: 'SET_ITEMS_FILTER'; payload: string };

const initialState: InventoryState = {
  places: [],
  items: [],
  error: null,
  itemsFilter: '',
};

const placeReducer = (
  state: InventoryState,
  action: InventoryAction
): InventoryState => {
  switch (action.type) {
    case 'SET_PLACES':
      return { ...state, places: action.payload };
    case 'SET_ITEMS':
      return { ...state, items: action.payload };
    case 'SET_ERROR':
      return { ...state, error: action.payload };
    case 'SET_ITEMS_FILTER':
      return { ...state, itemsFilter: action.payload };
    default:
      return state;
  }
};

export const PlacesPage = () => {
  const [state, dispatch] = React.useReducer(
    placeReducer,
    initialState
  );
  const { places, items, error, itemsFilter } = state;

  const columns: GridColDef[] = [
    { field: 'nbOfItems', headerName: 'Qte', width: 80 },
    {
      field: 'placeName',
      headerName: 'Lieux',
      width: 270,
      renderCell: (params) => {
        const place = params.row as InventoryPlace;
        return (
          <Box display="flex" alignItems="center">
            {place.placeType && (
              <Chip
                label={place.placeTypeFrench}
                size="small"
                sx={{
                  mr: 0.5,
                  backgroundColor: getPlaceTypeColor(
                    translatePlaceTypeFR(place.placeType)
                  ),
                  color: CHIP_PROPERTIES.color,
                  border: CHIP_PROPERTIES.border,
                  height: `${CHIP_PROPERTIES.height}px`,
                  width: `${CHIP_PROPERTIES.width}px`,
                }}
              />
            )}
            <Typography>{place.placeName}</Typography>
          </Box>
        );
      },
    },
  ];

  const handleFetchItems = async (path: string) => {
    try {
      const resultDto = await getResponse<ItemDto[]>(path, undefined);
      if (resultDto instanceof Error) {
        dispatch({
          type: 'SET_ERROR',
          payload:
            'Oopsie...Vérifier la connexion Internet et rafraîchir la page.',
        });
      } else {
        const items = resultDto.map(buildItem);
        dispatch({ type: 'SET_ITEMS', payload: items });
      }
    } catch {
      dispatch({
        type: 'SET_ERROR',
        payload: 'Something went wrong...',
      });
    }
  };

  React.useEffect(() => {
    handleFetchItems('items');
  }, []);

  const handleFetchPlaces = async (
    path: string,
    filter: { [key: string]: string } | undefined
  ) => {
    try {
      const resultDto = await getResponse<InventoryPlaceDto[]>(
        path,
        filter
      );
      if (resultDto instanceof Error) {
        dispatch({
          type: 'SET_ERROR',
          payload:
            'Oopsie...Vérifier la connexion Internet et rafraîchir la page.',
        });
      } else {
        const places = resultDto.map(buildInventoryPlace);
        dispatch({ type: 'SET_PLACES', payload: places });
      }
    } catch {
      dispatch({
        type: 'SET_ERROR',
        payload: 'Something went wrong...',
      });
    }
  };

  const [selectedPlaceTypes, setSelectedPlaceTypes] = React.useState<
    string[]
  >(ALL_PLACE_TYPES_FR);

  React.useEffect(() => {
    const filters: { [key: string]: string } = {};
    if (itemsFilter !== 'Tous' && itemsFilter !== '') {
      filters.item_name = itemsFilter;
    }
    if (selectedPlaceTypes.length > 0) {
      dispatch({
        type: 'SET_ERROR',
        payload: '',
      });
      filters.place_type = selectedPlaceTypes
        .map(translatePlaceTypeEN)
        .join(',');
    } else {
      dispatch({
        type: 'SET_ERROR',
        payload:
          "Il faut sélectionner au moins un type d'emplacement.",
      });
    }
    handleFetchPlaces('inventory/places', filters);
  }, [itemsFilter, selectedPlaceTypes]);

  const handleCheckboxChange = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const { value, checked } = event.target;
    setSelectedPlaceTypes((prev) =>
      checked
        ? [...prev, value]
        : prev.filter((type) => type !== value)
    );
  };

  return (
    <>
      <Box
        display="flex"
        alignItems="left"
        justifyContent="left"
        sx={{ mb: 1, mt: 1 }}
      >
        <ChipFilter
          chips={ALL_PLACE_TYPES_FR}
          selectedChips={selectedPlaceTypes}
          handleCheckboxChange={handleCheckboxChange}
          getChipColor={getPlaceTypeColor}
          showDeleteIcon={true}
          chipProperties={CHIP_PROPERTIES}
        />
      </Box>
      <Box
        display="flex"
        alignItems="left"
        justifyContent="left"
        sx={{ mb: 1 }}
      >
        <SelectFilter
          label="Type d'objet"
          options={items}
          optionLabelHandler={(option) => option.itemName}
          onChangeHandler={(value) =>
            dispatch({
              type: 'SET_ITEMS_FILTER',
              payload: value as string,
            })
          }
        />
      </Box>
      <CustomDataGrid
        columns={columns}
        rows={places}
        error={error}
        getRowId={(place: InventoryPlace) => place.placeId}
      />
    </>
  );
};
