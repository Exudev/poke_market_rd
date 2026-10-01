# PokeMarketRD

PokeMarketRD es un marketplace en línea premium dedicado a la venta de cartas coleccionables de Pokémon en República Dominicana. Su objetivo es conectar a coleccionistas y aficionados locales con vendedores confiables en un entorno seguro y con una interfaz "premium" limpia, fluida y moderna.

## Características Principales

*   **Diseño Moderno y Premium:** Interfaz minimalista inspirada en la filosofía de diseño tipo Apple, pero con acentos de color propios del universo Pokémon (rojo vivo `#E53822`, azul real `#0B4DA2`, y un toque de amarillo `#E4AA34`).
*   **Catálogo de Productos:** Exploración de cartas con información detallada, imágenes de alta calidad (simuladas en esta versión), rareza, set, y estado de la carta (ej. NM, Mint).
*   **Carrito de Compras:** Sistema funcional de carrito mediante React Context API, que maneja persistencia, cantidad y cálculos de precios.
*   **Checkout Integrado:** Formularios de facturación limpios con integración de las pasarelas de pago más reconocidas (`PayPal` y `Google Pay`), proveyendo una UX fluida y directa.
*   **Adaptado a RD:** Detalles adaptados al contexto de República Dominicana, como el cálculo del ITBIS (18%) y enfoque en envíos y localidades nacionales.

## Tecnologías Utilizadas

*   **Frontend:** React 19, TypeScript, React Router.
*   **Estilos:** Tailwind CSS (versión v4) para utility-first styling, paletas personalizadas, y animaciones fluidas.
*   **Animaciones:** Framer Motion (v12) para transiciones limpias y entradas suaves (`motion/react`).
*   **Pagos:** 
    * `@paypal/react-paypal-js` para botones nativos de PayPal.
    * `@google-pay/button-react` para la integración de Google Pay directo en la web.
*   **Iconos:** Lucide React para iconografía consistente, ligera y expresiva.
*   **Utilidades:** `clsx` y `tailwind-merge` para el manejo dinámico y seguro de clases.

## Estructura del Proyecto

*   `src/components/layout`: Componentes de estructura global (Navbar, Footer, Layout principal).
*   `src/components/ui`: Componentes reusables de interfaz (Button, Badge).
*   `src/context/CartContext.tsx`: Manejo del estado global del carrito.
*   `src/data/mockData.ts`: Información ficticia de las cartas iniciales para el MVP.
*   `src/pages`: Las distintas vistas y pantallas principales (Home, Catalog, ProductDetail, Cart, Checkout).

## Instrucciones

La aplicación corre en un entorno de desarrollo en Vite. Asegúrate de tener las dependencias instaladas y ejecuta `npm run dev` para iniciar el servidor. 

> *Nota: Las transacciones de PayPal se encuentran en modo Sandbox ("test") y las de Google Pay están configuradas bajo environment "TEST".*
