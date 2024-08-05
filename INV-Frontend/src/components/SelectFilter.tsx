import * as React from 'react';
import { Autocomplete, TextField, Box } from '@mui/material';

type SelectFilter<T> = {
  options: T[];
  optionLabelHandler: (option: T) => string;
  onChangeHandler: (value: React.SetStateAction<string>) => void;
  label: string;
  renderOption?: (
    props: React.HTMLAttributes<HTMLLIElement>,
    option: T
  ) => React.ReactNode;
};

export const SelectFilter = <T,>({
  options: options,
  optionLabelHandler: optionLabelHandler,
  onChangeHandler: onChangeHandler,
  label,
  renderOption,
}: SelectFilter<T>) => {
  const [optionName, setOptionName] = React.useState('');
  const allOption = { label: 'Tous', value: '' } as unknown as T;
  const extendedOptions = [allOption, ...options];

  return (
    <Box sx={{ width: '100%', alignContent: 'flex-start' }}>
      <Autocomplete
        id="combo-box-demo"
        renderInput={(params) => (
          <TextField
            {...params}
            label={label}
            placeholder={`Choisissez un ${label.toLowerCase()}`}
          />
        )}
        options={options}
        getOptionLabel={optionLabelHandler}
        value={
          extendedOptions.find(
            (option) => optionLabelHandler(option) === optionName
          ) || null
        }
        onChange={(_event, newValue) => {
          setOptionName(newValue ? optionLabelHandler(newValue) : '');
          onChangeHandler(
            newValue ? optionLabelHandler(newValue) : ''
          );
        }}
        noOptionsText="Aucune option"
        fullWidth
        blurOnSelect={true}
        renderOption={renderOption}
      />
    </Box>
  );
};
