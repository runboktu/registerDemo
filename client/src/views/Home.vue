<template>
  <div class="home-container">
    <div v-if="user" class="welcome-section">
      <h2>Welcome, {{ user.username }}!</h2>
      <div class="user-info">
        <p><strong>Email:</strong> {{ user.email }}</p>
        <p><strong>User ID:</strong> {{ user.id }}</p>
        <p><strong>Created At:</strong> {{ new Date(user.created_at).toLocaleString() }}</p>
      </div>
      <button @click="handleLogout" class="logout-btn">Logout</button>
    </div>
    <div v-else class="auth-links">
      <h2>Welcome to Auth Demo</h2>
      <p>Please login or register to continue</p>
      <div class="buttons">
        <router-link to="/login" class="btn login-btn">Login</router-link>
        <router-link to="/register" class="btn register-btn">Register</router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const user = ref(null)

onMounted(() => {
  const userStr = localStorage.getItem('user')
  if (userStr) {
    user.value = JSON.parse(userStr)
  }
})

const handleLogout = () => {
  localStorage.removeItem('token')
  localStorage.removeItem('user')
  user.value = null
  router.push('/login')
}
</script>

<style scoped>
.home-container {
  max-width: 800px;
  margin: 40px auto;
  padding: 20px;
  text-align: center;
}

.welcome-section {
  background-color: #f9f9f9;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.user-info {
  text-align: left;
  margin: 20px 0;
  padding: 20px;
  background-color: white;
  border-radius: 4px;
}

.logout-btn {
  background-color: #f44336;
  color: white;
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
}

.logout-btn:hover {
  background-color: #d32f2f;
}

.auth-links {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.buttons {
  display: flex;
  gap: 20px;
  justify-content: center;
}

.btn {
  padding: 10px 20px;
  border-radius: 4px;
  text-decoration: none;
  font-weight: bold;
}

.login-btn {
  background-color: #4CAF50;
  color: white;
}

.register-btn {
  background-color: #2196F3;
  color: white;
}

.btn:hover {
  opacity: 0.9;
}
</style> 