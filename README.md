# 🇩🇴 PokéMart DR

The #1 Pokémon TCG Marketplace in the Dominican Republic. Buy, sell, and trade Pokémon cards safely with local collectors.

![PokéMart Hero](./frontend/public/favicon.svg)

## 🚀 Features

- **Robust Marketplace**: List your Pokémon cards with conditions, sets, images, and prices.
- **Live Market Data**: Integrates directly with `api.pokemontcg.io` to provide real-time market prices for cards.
- **In-App Offers**: Buyers can make direct cash offers to sellers. When accepted, the seller's WhatsApp number is unlocked to finalize the deal.
- **Community Feed**: A dedicated social feed to share pulls, ask questions, and chat about the hobby.
- **AI Image Moderation**: Powered by Cloudinary & AWS Rekognition to automatically reject inappropriate images before they hit the database.
- **Global Error Handling**: Predictable, clean HTTP error codes and beautiful Toast notifications for all user actions.
- **Mobile-First UX**: Features an app-like sticky bottom navigation bar and sleek empty states.
- **Secure Authentication**: JWT-based auth, email verification via Resend, and forgotten password flows.

## 🛠 Tech Stack

### Frontend
- **Framework:** Vue 3 (Composition API) + Vite
- **Styling:** Tailwind CSS
- **Icons:** Lucide Vue
- **Routing:** Vue Router
- **HTTP Client:** Axios

### Backend
- **Language:** Rust 🦀
- **Framework:** Axum
- **Database:** PostgreSQL (with `sqlx` for compile-time verified queries)
- **Authentication:** JWT (JSON Web Tokens), bcrypt hashing
- **Email:** Resend API + Lettre
- **Storage:** Cloudinary (with AWS Rekognition auto-moderation)

---

## 💻 Local Development

### Prerequisites
- Node.js (v18+)
- Rust (cargo)
- PostgreSQL running locally

### 1. Database Setup
```bash
# Create a local postgres database
createdb poke_market
```

### 2. Backend Setup
```bash
cd backend

# Create a .env file
cp .env.example .env
# Edit .env and add your PostgreSQL URL, Cloudinary URL, and Resend credentials

# Run database migrations
cargo sqlx migrate run

# Start the Rust server
cargo run
```

### 3. Frontend Setup
```bash
cd frontend

# Install dependencies
npm install

# Start the Vite development server
npm run dev
```

## 🔒 Environment Variables Reference

**Backend (`backend/.env`)**
```env
# Database
DATABASE_URL=postgres://username:password@localhost/poke_market

# Authentication & Server
JWT_SECRET=super_secret_jwt_key
PORT=8080

# Frontend Base URL (Used for verification & password reset email links)
# - Local: http://localhost:5173
# - Production: https://your-domain.com
BASE_URL=http://localhost:5173

# Optional: Server URL fallback for local uploads (if Cloudinary is omitted)
# BACKEND_URL=http://localhost:8080

# Cloudinary (Optional, for card photo uploads & AWS Rekognition auto-moderation)
CLOUDINARY_URL=cloudinary://API_KEY:API_SECRET@CLOUD_NAME

# Resend / SMTP Email (Leave empty in local dev to use terminal STUB output)
SMTP_HOST=smtp.resend.com
SMTP_USER=resend
SMTP_PASS=re_your_resend_api_key
SMTP_FROM=PokéMart <onboarding@resend.dev>
```

**Frontend (`frontend/.env`)**
```env
# Backend API URL
# - Local: http://localhost:8080
# - Production: https://api.your-domain.com
VITE_API_URL=http://localhost:8080
```

---

## 📜 License
MIT License. Created for the Dominican Pokémon TCG Community.
