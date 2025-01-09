import axios from 'axios'

const api = axios.create({
  baseURL: 'http://localhost:8080/api'
})

export const register = (userData) => {
  return api.post('/register', userData)
}

export const login = (credentials) => {
  return api.post('/login', credentials)
}

export default api 