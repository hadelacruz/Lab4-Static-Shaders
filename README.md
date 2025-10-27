# 🌍 Lab4 - Static Shaders: Sistema de Planetas Procedurales

**Laboratorio de Gráficas por Computadora**  
*Generación procedural de planetas usando shaders personalizados en Rust*

---

## 📋 Tabla de Contenidos

1. [Descripción del Proyecto](#-descripción-del-proyecto)
2. [Capturas de Pantalla](#-capturas-de-pantalla)
3. [Características Principales](#-características-principales)
4. [Cómo Ejecutar el Programa](#-cómo-ejecutar-el-programa)
5. [Controles](#-controles)
6. [Arquitectura del Sistema](#-arquitectura-del-sistema)
7. [Documentación Técnica](#-documentación-técnica)
8. [Planetas Implementados](#-planetas-implementados)
9. [Créditos](#-créditos)

---

## 🚀 Descripción del Proyecto

Este proyecto implementa un sistema de renderizado de planetas procedurales usando **shaders personalizados** escritos completamente en Rust. Cada planeta se genera mediante algoritmos de ruido sin usar texturas pregrabadas.

**Tecnologías:**
- **Rust** - Lenguaje de programación
- **Raylib 5.0** - Framework gráfico
- **Renderizado por Software** - Pipeline completo sin GPU shaders
- **Generación Procedural** - Algoritmos de ruido (Perlin, FBM, Voronoi, Ridge)

---

## 📸 Capturas de Pantalla

### Planeta 1: Rocoso Gris con Relieve Procedural

> **[ESPACIO PARA CAPTURA]**
> 
> *Presiona tecla **1** para ver este planeta*

**Características:**
- Deformación geométrica procedural (montañas, colinas, cráteres)
- Paleta de 7 tonos de gris
- Fracturas y vetas minerales

---

### Planeta 2: Gigante Gaseoso

> **[ESPACIO PARA CAPTURA]**
> 
> *Presiona tecla **2** para ver este planeta*

**Características:**
- Bandas atmosféricas horizontales suaves
- Gran Mancha Roja con rotación espiral
- Turbulencia multicapa

---

### Planeta 3: Sci-Fi Tecnológico

> **[ESPACIO PARA CAPTURA]**
> 
> *Presiona tecla **3** para ver este planeta*

**Características:**
- Patrón hexagonal de circuitos
- Líneas de escaneo animadas
- Glitches aleatorios y pulsos de energía

---

### Planeta 4: Nebulosa Cósmica

> **[ESPACIO PARA CAPTURA]**
> 
> *Presiona tecla **4** para ver este planeta*

**Características:**
- Ondas de choque expansivas
- Vórtices energéticos
- Paleta cósmica (púrpura, magenta, naranja, cian)

---

## ✨ Características Principales

- ✅ **Vertex Shaders** - Deformación procedural de geometría
- ✅ **Fragment Shaders** - Coloreado y texturizado por píxel
- ✅ **4 Planetas Únicos** - Cada uno con efectos especiales diferentes
- ✅ **Fondo Espacial** - Galaxia con estrellas animadas
- ✅ **Iluminación Realista** - Difusa, especular, rim lighting
- ✅ **100% Procedural** - Sin texturas pregrabadas

---

## 🎮 Cómo Ejecutar el Programa

### Requisitos Previos

- **Rust** instalado (https://www.rust-lang.org/tools/install)
- **Git** para clonar el repositorio

### Instalación

```bash
# Clonar repositorio
git clone https://github.com/hadelacruz/Lab4-Static-Shaders.git
cd Lab4-Static-Shaders

# Compilar
cargo build --release

# Ejecutar
cargo run --release
```

**⚠️ Importante:** Siempre usar `--release` para mejor rendimiento.

---

## 🎮 Controles

| Tecla | Acción |
|-------|--------|
| **1** | Ver Planeta Rocoso |
| **2** | Ver Gigante Gaseoso |
| **3** | Ver Planeta Sci-Fi |
| **4** | Ver Planeta Nebulosa |
| **ESC** | Salir |

---

## 🏗️ Arquitectura del Sistema

### Estructura del Proyecto

```
Lab4-Static-Shaders/
├── src/
│   ├── main.rs           # Loop principal y fondo espacial
│   ├── shaders.rs        # 4 shaders de planetas
│   ├── camera.rs         # Cámara orbital
│   ├── sphere.rs         # Cargador de modelos OBJ
│   ├── sphere.obj        # Esfera (1890 vértices, 960 triángulos)
│   ├── vector.rs         # Vector3D
│   ├── matrix.rs         # Matrices de transformación
│   ├── vertex.rs         # Estructura de vértice
│   ├── fragment.rs       # Procesamiento de fragmentos
│   ├── framebuffer.rs    # Buffer de píxeles
│   ├── triangle.rs       # Rasterización
│   └── render.rs         # Pipeline de renderizado
├── Cargo.toml            # Dependencias
└── README.md             # Documentación
```

### Pipeline de Renderizado

```
1. Cargar Modelo (sphere.obj)
        ↓
2. Para cada Frame:
   ├─ Renderizar fondo (galaxia + estrellas)
   ├─ Para cada triángulo:
   │  ├─ Aplicar rotación del planeta
   │  ├─ VERTEX SHADER → Deformar geometría
   │  ├─ Transformar a espacio de cámara
   │  ├─ Proyectar a pantalla 2D
   │  ├─ Rasterizar triángulo
   │  └─ Para cada píxel:
   │     ├─ Interpolar posición, normal, UV
   │     ├─ FRAGMENT SHADER → Calcular color
   │     └─ Escribir en framebuffer (con Z-buffer)
   └─ Dibujar UI (controles)
```

### Sistema de Shaders

```rust
// Trait común para todos los planetas
pub trait PlanetShader {
    // Modifica geometría (vertex shader)
    fn vertex_shader(
        &self, 
        position: Vector3,     // Posición 3D del vértice
        normal: Vector3,       // Normal del vértice
        uv: (f32, f32),       // Coordenadas de textura
        uniforms: &ShaderUniforms
    ) -> (Vector3, Vector3);  // Nueva posición y normal
    
    // Calcula color (fragment shader)
    fn fragment_shader(
        &self,
        position: Vector3,     // Posición 3D del píxel
        normal: Vector3,       // Normal interpolada
        uv: (f32, f32),       // UV interpoladas
        uniforms: &ShaderUniforms
    ) -> ShaderColor;         // Color RGBA
}
```

### Uniforms Globales

Parámetros compartidos entre todos los shaders:

```rust
pub struct ShaderUniforms {
    pub time: f32,              // Tiempo para animaciones
    pub light_direction: Vector3,  // Dirección del sol
    pub camera_position: Vector3,  // Posición de cámara
}
```

---

## 📚 Documentación Técnica

### Funciones de Ruido Procedural

| Función | Descripción | Uso Principal |
|---------|-------------|---------------|
| `perlin_noise(x, y, z)` | Ruido 3D suave y continuo | Base para terrenos |
| `fbm(x, y, octaves)` | Múltiples capas de ruido 2D | Texturas complejas |
| `fbm3d(x, y, z, octaves)` | Múltiples capas de ruido 3D | Deformaciones volumétricas |
| `voronoi_noise(x, y)` | Patrones celulares | Cráteres, cristales |
| `ridge_noise(x, y, octaves)` | Crestas y líneas | Montañas, venas |
| `smoothstep(a, b, x)` | Interpolación suave (curva S) | Transiciones graduales |
| `mix(a, b, t)` | Interpolación lineal | Mezcla de valores |

### Ejemplos de Uso

**Terreno con múltiples detalles:**
```rust
let mountains = fbm3d(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0, 4) * 0.15;
let hills = fbm3d(pos.x * 5.0, pos.y * 5.0, pos.z * 5.0, 3) * 0.08;
let details = fbm3d(pos.x * 15.0, pos.y * 15.0, pos.z * 15.0, 2) * 0.03;
let displacement = mountains + hills + details;
```

**Cráteres procedurales:**
```rust
let pattern = voronoi_noise(uv.0 * 8.0, uv.1 * 8.0);
if pattern < 0.2 {
    // Es un cráter
    let depth = -0.05 * (1.0 - pattern / 0.2);
}
```

**Transiciones de color suaves:**
```rust
let color = if value < 0.5 {
    mix_color(dark, medium, smoothstep(0.0, 0.5, value))
} else {
    mix_color(medium, bright, smoothstep(0.5, 1.0, value))
};
```

### Técnicas de Iluminación

| Técnica | Fórmula | Efecto |
|---------|---------|--------|
| **Difusa (Lambert)** | `max(dot(normal, light), 0.0)` | Luz básica direccional |
| **Especular (Phong)** | `pow(max(dot(reflect, view), 0.0), shininess)` | Brillos metálicos |
| **Rim Lighting** | `pow(1.0 - abs(dot(view, normal)), exp)` | Halo en bordes |
| **Wrap Diffuse** | `dot(normal, light) * 0.5 + 0.5` | Luz envolvente (atmósferas) |
| **Subsurface Scattering** | `(1.0 + dot(normal, light)) * 0.25` | Luz atravesando material |

---

## 🌍 Planetas Implementados

### 1️⃣ Planeta Rocoso (`RockyPlanetShader`)

**Vertex Shader - 5 Capas de Deformación:**
- Montañas (escala 2.0, amplitud 0.15)
- Colinas (escala 5.0, amplitud 0.08)
- Detalles (escala 15.0, amplitud 0.03)
- Cráteres (Voronoi, depresión -0.05)
- Pulso tectónico animado (0.01)

**Fragment Shader:**
- **Paleta:** 7 tonos de gris (RGB 20-200)
- **Efectos:** Erosión, fracturas, vetas minerales
- **Iluminación:** Difusa + Especular (exp 8) + Oclusión ambiental

---

### 2️⃣ Gigante Gaseoso (`GasGiantShader`)

**Fragment Shader:**
- **Bandas:** 8 frecuencia, transiciones suaves
- **Turbulencia:** 4 capas (escalas 5, 8, 12, 20)
- **Vórtice:** Gran Mancha Roja con rotación espiral
- **Paleta:** 8 tonos (marrón → naranja → crema)
- **Iluminación:** Wrap diffuse (0.3/0.7) + Subsurface + Rim

---

### 3️⃣ Planeta Sci-Fi (`CrystalPlanetShader`)

**Vertex Shader:**
- Pulso de energía: `sin(time * 3.0 + length * 5.0) * 0.02`

**Fragment Shader:**
- **Efectos:** Hexágonos (Voronoi), líneas de escaneo, glitches
- **Paleta:** Azul, cian, púrpura, rosa neón
- **Animación:** Flujo de datos, pulsos a 4Hz

---

### 4️⃣ Planeta Nebulosa (`LavaPlanetShader`)

**Vertex Shader:**
- Ondas de choque: `sin(dist * 5.0 - time * 3.0) * 0.03`

**Fragment Shader:**
- **Efectos:** Vórtices energéticos, partículas de estrellas
- **Paleta:** Púrpura, magenta, naranja, cian
- **Iluminación:** Auto-iluminación 0.6 (emisivo puro)
