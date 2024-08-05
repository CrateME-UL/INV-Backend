import * as React from 'react';
import Box from '@mui/material/Box';
import FormControl from '@mui/material/FormControl';
import FormGroup from '@mui/material/FormGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import Checkbox from '@mui/material/Checkbox';
import FormLabel from '@mui/material/FormLabel';
import Chip from '@mui/material/Chip';
import Fade from '@mui/material/Fade';
interface ChipFilterProps {
  chips: string[];
  selectedChips: string[];
  handleCheckboxChange?: (
    event: React.ChangeEvent<HTMLInputElement>
  ) => void;
  getChipColor: (Chip: string) => string;
  showDeleteIcon: boolean;
  chipProperties: {
    height: number;
    width: number;
    color: string;
    border: string;
  };
}

const ChipFilter: React.FC<ChipFilterProps> = ({
  chips: chips,
  selectedChips,
  handleCheckboxChange,
  getChipColor,
  showDeleteIcon,
  chipProperties: chipProperties,
}) => {
  return (
    <Box
      alignItems="center"
      justifyContent="left"
      sx={{
        border: '1px solid rgba(0, 0, 0, 0.23)',
        padding: '10px',
        position: 'relative',
        width: '100%',
        borderRadius: '4px',
      }}
    >
      <FormLabel
        component="legend"
        sx={{
          position: 'absolute',
          top: '-10px',
          left: '10px',
          backgroundColor: 'white',
        }}
      >
        Type d'emplacement
      </FormLabel>
      <FormControl component="fieldset">
        <FormGroup row={true}>
          {chips.map((type) => (
            <Box key={type} display="flex" alignItems="left" mr={-2}>
              <FormControlLabel
                control={
                  <Checkbox
                    checked={selectedChips.includes(type)}
                    onChange={handleCheckboxChange}
                    value={type}
                    disabled={!showDeleteIcon}
                    icon={
                      <Fade
                        in={!selectedChips.includes(type)}
                        timeout={300}
                        exit={true}
                      >
                        <Chip
                          label={type}
                          size="small"
                          sx={{
                            backgroundColor: 'transparent',
                            color: chipProperties.color,
                            border: chipProperties.border,
                            height: `${chipProperties.height}px`,
                            width: `${chipProperties.width}px`,
                          }}
                        />
                      </Fade>
                    }
                    checkedIcon={
                      <Fade
                        in={selectedChips.includes(type)}
                        timeout={300}
                        exit={true}
                      >
                        <Chip
                          label={type}
                          size="small"
                          sx={{
                            backgroundColor: getChipColor(type),
                            color: chipProperties.color,
                            border: chipProperties.border,
                            height: `${chipProperties.height}px`,
                            width: showDeleteIcon
                              ? `${chipProperties.width + 20}px`
                              : chipProperties.width,
                            '& .MuiChip-deleteIcon': {
                              color: chipProperties.color,
                              fontSize: `${
                                chipProperties.height - 10
                              }px`,
                              cursor: 'pointer',
                            },
                          }}
                          onDelete={
                            showDeleteIcon
                              ? handleCheckboxChange
                                ? () =>
                                    handleCheckboxChange({
                                      target: { value: type },
                                    } as React.ChangeEvent<HTMLInputElement>)
                                : undefined
                              : undefined
                          }
                        />
                      </Fade>
                    }
                  />
                }
                label={undefined}
              />
            </Box>
          ))}
        </FormGroup>
      </FormControl>
    </Box>
  );
};

export default ChipFilter;
