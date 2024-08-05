import ReactDOM from 'react-dom/client';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import GlobalStyles from '@mui/material/GlobalStyles';
import { Inventory } from './Inventory';

// Define a custom theme
export const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2', // Customize your primary color
    },
    secondary: {
      main: '#dc004e', // Customize your secondary color
    },
  },
  typography: {
    fontFamily: 'Roboto, Arial, sans-serif', // Customize your font family
  },
});

export default theme;

// Ensure the root element exists before attempting to render
const rootElement = document.getElementById('root');

if (rootElement) {
  // Create a root for React to render the application
  const root = ReactDOM.createRoot(rootElement);

  // Render the Inventory component within React.StrictMode (for dev) and ThemeProvider
  root.render(
    // <React.StrictMode>
    <ThemeProvider theme={theme}>
      <GlobalStyles
        styles={{
          body: { overflow: 'hidden' },
          html: { overflow: 'hidden' },
        }}
      />
      <Inventory />
    </ThemeProvider>
    // </React.StrictMode>
  );
} else {
  // Log an error if the root element is not found
  console.error(
    'Root element not found. Unable to render the application.'
  );
}
