# Link Inspector

Link Inspector es una aplicación web creada de manera colaborativa que puedes ejecutar localmente (sin dependencias adicionales, solo ejecutando un archivo) diseñada para encontrar hipervínculos o enlaces en los principales tipos de de archivos y comprobar su disponibilidad (funcionan, están caídos, redirigen a alguna otra dirección, etc.).

Es una herramienta completamente gratuita que puedes user sin ningún tipo con limitación y con total seguridad (puedes revisar el código para que veas que no hay nada "extraño"). 

## Uso de la aplicación

Para comenzar a usar Link Inspector, simplemente descarga el ejecutable disponible en la sección de [Releases](htts://github.com/antikorps/link_inspector/releases). Una vez descargado, sigue estos sencillos pasos:

1. **Ejecuta el archivo**: Dependiendo de tu sistema operativo, ejecuta el archivo descargado.
2. **Accede a la aplicación**: Al iniciar, la aplicación verás que aparece una URL en la terminal. Solo tienes que copiar esa URL en tu navegador para comenzar a utilizar Link Inspector.

## Información para desarrolladores y colaboradores
Este proyecto ha sido construido utilizando las siguientes tecnologías:
- **Frontend:** Astro (framework moderno para construir aplicaciones web rápidas).
- **Backend**: Rust utilizando el framework Axum.


## Comandos importantes
Si deseas contribuir al desarrollo de Link Inspector, aquí tienes algunas cuestiones que te ayudarán a mejorar la experiencia de desarrollo:
Levantar el backend: 
```
cargo run
```

Levantar el frontend en modo desarrollo (en el directorio frontend): 
```
npx astro dev
```
Para compilar el proyecto primero es necesario generar los archivos estáticos que forman el frontend y que se embeberán en el ejecutable final. Es decir, que habrá que seguir la siguiente secuencia:
Desde el directorio frontend
```
npx astro build
```
Desde el directorio raíz
```
cargo build --release
```


### Proxy en Vite Config
Una de las cuestiones que más ha contribuido a la mejora de la experiencia de desarrollo es el hot reload en el frontend. En este caso, como front y back están completamente desacoplados se ejecutan en puertos distintos, por lo que para redireccionar las peticiones del front al back sin tener que estar hardcodeando se recomienda el uso del archivo `vite.config.js` 

```js
import { defineConfig } from "vite";

export default defineConfig({
  server: {
    proxy: {
      "/upload": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
    },
  },
});
```

Con esa configuración, por ejemplo, se redireccionará la petición /upload al puerto 3000. Si quieres forzar la ejecución del backend en un puerto concreto puedes servirte de la variable de entorno **LINK_INSPECTOR_PORT**

## Contribuye y Colabora

¡Tu participación es valiosa! Ya seas desarrollador, diseñador, o simplemente tengas una buena idea, tu colaboración es bienvenida. Siéntete libre de abrir issues, realizar pull requests o simplemente dar feedback.