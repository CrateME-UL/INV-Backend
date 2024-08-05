import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';

interface ErrorPageProps {
  errorCode: number;
  errorMessage: string;
}

export const ErrorPage = ({
  errorCode,
  errorMessage,
}: ErrorPageProps) => {
  return (
    <Box
      sx={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'flex-start',
        height: 'auto',
        paddingTop: '2rem',
        flexDirection: 'column',
        backgroundColor: '#f5f5f5',
      }}
    >
      <Box sx={{ mb: 2 }}>
        <Typography variant="h1" component="h1" color="error">
          {errorCode}
        </Typography>
      </Box>
      <Box>
        <Typography variant="h2" component="h2" color="textSecondary">
          {errorMessage}
        </Typography>
      </Box>
    </Box>
  );
};
