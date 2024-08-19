import axios from 'axios';

const API_ENDPOINT = import.meta.env.VITE_API_ENDPOINT.replace(
  /\/?$/,
  ''
);

console.log(`GET ${API_ENDPOINT}`);

export const getResponse = async <T>(
  path: string,
  params?: { [key: string]: string } | undefined
): Promise<T | Error> => {
  try {
    console.log(`GET ${API_ENDPOINT}/${path}`);
    const result = await axios.get(
      `${API_ENDPOINT}/${path}`,
      params ? { params } : undefined
    );
    return result.data;
  } catch {
    return new Error('Failed to fetch data');
  }
};
