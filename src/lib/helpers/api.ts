const API_URL = "https://subfile-sigma.vercel.app/api";

export function create_url(endpoint: string) {
    return API_URL + endpoint;
}

import axios from 'axios';
import adapter from 'axios-tauri-api-adapter';

export const client = axios.create({ adapter: adapter });