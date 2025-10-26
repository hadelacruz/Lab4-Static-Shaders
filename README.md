# Lab 4 - Shaders de Planetas (Static Shaders)

## Descripción del Proyecto

Este proyecto implementa un sistema de renderizado de cuerpos celestes usando **únicamente shaders** en Rust. Se generan tres tipos de planetas diferentes sin usar texturas ni materiales precargados, todo el efecto visual proviene de cálculos procedurales en los fragment shaders.

## 🪐 Planetas Implementados

### 1. Planeta Rocoso
- **Características**: Superficie con variaciones de terreno tipo Marte/Tierra
- **Técnicas**: 
  - Ruido fractal (FBM - Fractional Brownian Motion) con 6 octavas
  - 4 capas de colores (marrón, verde oscuro, gris, beige)
  - Iluminación Lambertiana
  - Detalles finos mediante ruido de alta frecuencia
- **Colores**: Tonos terrosos y rocosos

### 2. Gigante Gaseoso
- **Características**: Bandas horizontales animadas tipo Júpiter
- **Técnicas**:
  - Bandas horizontales con turbulencia
  - Sistema de tormentas (Gran Mancha Roja)
  - Ruido procedimental animado con el tiempo
  - 4 gradientes de color (amarillo, naranja, marrón, crema)
- **Colores**: Tonos cálidos con efecto de tormenta rojiza

### 3. Planeta Cristalino (Ciencia Ficción)
- **Características**: Planeta de cristal con venas de energía pulsante
- **Técnicas**:
  - Patrón de cristales animados
  - Venas de energía que recorren la superficie
  - Efecto Fresnel (bordes brillantes)
  - Iluminación con rim light
  - Pulsación sincronizada con el tiempo
- **Colores**: Azul cristal, púrpura y cyan brillante

## 🏗️ Arquitectura del Proyecto

```
src/
├── main.rs              # Punto de entrada, loop principal, control de input
├── framebuffer.rs       # Buffer de color y profundidad
├── vertex.rs            # Estructuras de vértices y matrices de transformación
├── fragment.rs          # Shaders de fragmentos (los 3 planetas)
├── obj_loader.rs        # Cargador de archivos OBJ
├── triangle.rs          # Rasterización de triángulos
├── render.rs            # Pipeline de renderizado
└── camera.rs            # Sistema de cámara orbital

esfera.obj               # Modelo base (esfera)
```

## 🎨 Técnicas de Shader Implementadas

### Fragment Shaders
- **Ruido Procedimental**: Función de ruido basada en seno para generar patrones pseudo-aleatorios
- **FBM (Fractional Brownian Motion)**: Múltiples octavas de ruido para crear variaciones naturales
- **Gradientes de Color**: Interpolación entre múltiples colores basada en valores de ruido
- **Iluminación**:
  - Lambert (diffuse lighting)
  - Ambient lighting
  - Rim lighting (Fresnel)
- **Animación Temporal**: Uso del uniform `time` para crear movimiento en los patrones
- **Interpolación Baricéntrica**: Para suavizar normales y posiciones

### Vertex Shaders
- Transformaciones MVP (Model-View-Projection)
- Sistema de coordenadas homogéneas
- Cálculo de normales interpoladas

## 🎮 Controles

| Tecla | Acción |
|-------|--------|
| `1` | Mostrar Planeta Rocoso |
| `2` | Mostrar Gigante Gaseoso |
| `3` | Mostrar Planeta Cristalino |
| `ESC` | Salir de la aplicación |

## 🚀 Compilación y Ejecución

### Requisitos
- Rust 1.70 o superior
- Cargo (incluido con Rust)

### Instalación
```bash
# Clonar el repositorio (si aplica)
git clone <tu-repo>
cd Lab4-Static-Shaders

# Compilar y ejecutar
cargo run --release
```

### Compilar solo
```bash
cargo build --release
```

El ejecutable estará en `target/release/`

## 📦 Dependencias

```toml
nalgebra-glm = "0.18"  # Matemáticas (vectores, matrices)
minifb = "0.25"        # Ventana y buffer de píxeles
tobj = "4.0"           # Cargador de archivos OBJ
```

## 🔧 Detalles Técnicos

### Pipeline de Renderizado
1. **Vertex Shader**: Transforma vértices a espacio de pantalla
2. **Rasterización**: Genera fragmentos mediante coordenadas baricéntricas
3. **Fragment Shader**: Calcula color final por píxel
4. **Z-Buffer**: Manejo de profundidad para oclusión correcta

### Optimizaciones
- Z-buffering para evitar overdraw
- Target FPS de 60
- Cámara orbital automática
- Cooldown en input para evitar cambios accidentales

### Matemáticas Clave
- **Ruido**: `sin(x * 12.9898 + y * 78.233 + z * 45.164) * 43758.5453`
- **FBM**: Suma de múltiples octavas con amplitud decreciente
- **Fresnel**: `(1 - dot(N, V))^3` para efecto de borde

## 📊 Complejidad por Shader

### Planeta Rocoso
- ⭐⭐⭐ Complejidad Media
- 2 niveles de ruido FBM
- 4 capas de color
- Iluminación Lambert + ambient

### Gigante Gaseoso
- ⭐⭐⭐⭐ Complejidad Alta
- Bandas procedurales
- Sistema de tormentas con blend
- Turbulencia animada
- 4 gradientes de color

### Planeta Cristalino
- ⭐⭐⭐⭐⭐ Complejidad Muy Alta
- Patrón de cristales animado
- Venas de energía (3 funciones seno multiplicadas)
- Efecto Fresnel
- Pulsación temporal
- Rim lighting
- 3 capas de color + energía

## 🎓 Conceptos Aplicados

- **Shading Procedimental**: Generación de patrones sin texturas
- **Coordenadas Baricéntricas**: Interpolación suave
- **Transformaciones 3D**: Matrices MVP
- **Iluminación**: Modelos difuso y especular simplificados
- **Animación**: Uso de tiempo como uniform
- **Z-Buffering**: Oclusión correcta
- **Modularización**: Separación clara de responsabilidades

## 🐛 Debugging

Si el programa no carga:
1. Verificar que `esfera.obj` está en la raíz del proyecto
2. Comprobar que las dependencias están instaladas: `cargo check`
3. Usar `cargo build --release` para mejor rendimiento

## 📝 Notas de Implementación

- **Sin texturas**: Todo el color viene de cálculos matemáticos
- **Un solo modelo**: La esfera se reutiliza para los 3 planetas
- **Shaders modulares**: Cada shader es una función independiente
- **Cambio en tiempo real**: Se puede cambiar de planeta sin reiniciar

## 👤 Autor

Proyecto desarrollado para el curso de Gráficas por Computadora.

## 📄 Licencia

Este proyecto es de uso académico.
