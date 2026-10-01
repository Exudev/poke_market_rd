# PokeMarketRD - Documento de Traspaso para Agente IA (Handoff)

¡Hola, colega IA! Si estás leyendo este documento, has sido asignado para continuar con el desarrollo de **PokeMarketRD**, un marketplace premium de cartas Pokémon para República Dominicana. A continuación, encontrarás todo el contexto técnico y funcional necesario para retomar el trabajo sin fricciones.

## 1. Contexto del Proyecto
* **Producto:** Marketplace B2C/C2C para compra y venta de cartas de Pokémon.
* **Estado actual:** Frontend MVP finalizado (SPA). Flujos de compra, carrito y diseño UI/UX completados. Usa datos mockeados.
* **Diseño:** Tema "Elegant Dark" (Premium, Minimalista, Oscuro profundo).

## 2. Stack Tecnológico
* **Core:** React 19, TypeScript, Vite.
* **Enrutamiento:** `react-router-dom` (SPA routing).
* **Estilos:** Tailwind CSS v4. Se utiliza una estrategia *utility-first* inyectando variables CSS globales.
* **Componentes visuales y utilidades:**
  * `lucide-react` para iconos.
  * `motion/react` (Framer Motion) para animaciones suaves (ver `Home.tsx`).
  * `clsx` y `tailwind-merge` para gestión de clases condicionales (ver `src/lib/utils.ts`).
* **Integraciones de Pago (Mock/Sandbox):**
  * `@paypal/react-paypal-js` (PayPal).
  * `@google-pay/button-react` (Google Pay).

## 3. Sistema de Diseño (Design System)
El proyecto utiliza un tema **Elegant Dark**. Siempre debes utilizar las variables CSS definidas en `src/index.css` en lugar de colores estáticos de Tailwind (e.g., no uses `bg-red-500`, usa `bg-[var(--color-accent)]`).

**Variables disponibles:**
* `--color-bg-base`: Fondo principal (`#0d0d0d`).
* `--color-bg-card`: Fondo de tarjetas/contenedores elevados (`#161616`).
* `--color-accent`: Color de acento primario (Botones, links, highlights - `#3b82f6`).
* `--color-accent-hover`: Hover del color primario (`#60a5fa`).
* `--color-text-main`: Texto principal (`#ffffff`).
* `--color-text-muted`: Texto secundario / subtítulos (`#a1a1aa`).
* `--color-border-main`: Bordes y separadores (`#27272a`).
* `--font-sans`: Tipografía principal (`Helvetica Neue`, Arial).

## 4. Estructura de Carpetas
```text
/src
  /components
    /layout     # Navbar.tsx, Footer.tsx, Layout.tsx
    /ui         # Button.tsx, Badge.tsx (Componentes base)
  /context      # CartContext.tsx (Gestión global de estado del carrito)
  /data         # mockData.ts (Base de datos front-end simulada)
  /pages        # Home.tsx, Catalog.tsx, ProductDetail.tsx, Cart.tsx, Checkout.tsx
  /lib          # utils.ts (Función cn para Tailwind)
```

## 5. Próximos Pasos (Backlog para el Agente)
Para llevar este proyecto a Producción, las siguientes tareas deben ser abordadas:

1. **Integración Backend / Base de Datos:**
   * Reemplazar `mockData.ts` con una base de datos real (ej. Firebase Firestore o Supabase). Usa `default_api:set_up_firebase` si el usuario da el visto bueno.
   * Crear el modelo de datos real para `Products`, `Users`, y `Orders`.
2. **Autenticación (Auth):**
   * Implementar inicio de sesión manejando los roles de Administrador, Vendedor y Comprador (Firebase Auth o Supabase Auth).
3. **Panel de Vendedor (Dashboard):**
   * Crear las pantallas necesarias para que los usuarios puedan subir y editar sus cartas (Gestión de Inventario CRUD).
4. **Pasarelas de Pago Locales (Opcional):**
   * Según el PRD original, la plataforma debe soportar pagos en República Dominicana (Azul, Cardnet). Actualmente cuenta con PayPal y Google Pay funcionales en el Front. Se requiere orquestar el backend transaccional.

## Instrucciones Operativas para ti (Agente)
* Preserva siempre la estructura de componentes en `/components/ui/`.
* No modifiques el archivo `index.css` ni destruyas las variables del tema Unless the user specifically requests a theme change.
* Manten el diseño responsivo usando las clases `sm:`, `md:`, `lg:` de Tailwind.
* Antes de hacer push de código nuevo, asegúrate de mantener el performance y evitar side-effects en React. Usa validaciones estrictas de Typescript.
