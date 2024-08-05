import { useState, useEffect } from 'react';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import { DataGrid, GridColDef, frFR } from '@mui/x-data-grid';
import Fade from '@mui/material/Fade';
import WarningIcon from '@mui/icons-material/Warning';
import ErrorIcon from '@mui/icons-material/Error';

type DataGridProps<T> = {
  error: string | null;
  rows: T[];
  columns: GridColDef[];
  getRowId: (row: T) => string | number;
};

export const CustomDataGrid = <T,>({
  error,
  rows: initialRows,
  columns,
  getRowId,
}: DataGridProps<T>) => {
  const [loading, setLoading] = useState(true);
  const [rows, setRows] = useState<T[]>([]);

  useEffect(() => {
    setLoading(true);
    setTimeout(() => {
      setRows(initialRows);
      setLoading(false);
    }, 50); // camouflage visual glitch
  }, [initialRows]);

  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
      }}
    >
      {loading ? (
        <Box
          display="flex"
          alignItems="center"
          justifyContent="center"
          sx={{ flexGrow: 1, mt: 3 }}
        ></Box>
      ) : (
        <>
          <Fade in={!loading && !!error} timeout={300} exit={true}>
            <Box
              display={error ? 'flex' : 'none'}
              alignItems="center"
              justifyContent="left"
              sx={{ m: 1.5 }}
            >
              <ErrorIcon
                sx={{
                  mr: 1,
                }}
              />
              <Typography component="span">{error}</Typography>
            </Box>
          </Fade>
          <Fade
            in={!loading && !error && rows.length === 0}
            timeout={300}
            exit={true}
          >
            <Box
              display={!error && rows.length === 0 ? 'flex' : 'none'}
              alignItems="left"
              justifyContent="left"
              sx={{ m: 1.5 }}
            >
              <WarningIcon
                sx={{
                  mr: 1,
                }}
              />
              <Typography component="span">
                Aucun résultat.
              </Typography>
            </Box>
          </Fade>
          <Fade
            in={!loading && !error && rows.length > 0}
            timeout={300}
            exit={true}
          >
            <Box sx={{ flexGrow: 1 }}>
              <DataGrid
                rows={rows}
                columns={columns}
                getRowId={getRowId}
                initialState={{
                  pagination: {
                    paginationModel: { page: 0, pageSize: 5 },
                  },
                }}
                disableColumnMenu
                pageSizeOptions={[5, 10]}
                localeText={
                  frFR.components.MuiDataGrid.defaultProps.localeText
                }
              />
            </Box>
          </Fade>
        </>
      )}
    </Box>
  );
};
