# Link Inspector

Link Inspector es una aplicación web creada de manera colaborativa que puedes ejecutar localmente (sin dependencias adicionales, solo ejecutando un archivo) diseñada para encontrar hipervínculos o enlaces en los principales tipos de de archivos y comprobar su disponibilidad (funcionan, están caídos, redirigen a alguna otra dirección, etc.).

Es una herramienta completamente gratuita que puedes user sin ningún tipo con limitación y con total seguridad (puedes revisar el código para que veas que no hay nada "extraño"). 

## Uso de la aplicación

Para comenzar a usar Link Inspector, simplemente descarga el ejecutable disponible en la sección de [Releases](htts://github.com/antikorps/link_inspector/releases) que corresponda a tu sistema operativo y arquitectura. Una vez descargado, sigue estos sencillos pasos:

1. **Ejecuta el archivo** con un doble clic (dependiendo del sistema operativo quizá sea previamente necesario otorgar al fichero permisos de ejecución).
2. **Accede a la aplicación** desde la dirección web que aparece en la consola. Generalmente solo será necesario hacer clic sobre el enlace, aunque tal vez sea necesario presionar control a la vez. Si estas opciones no funcionan, basta con copiar la URL en el navegador para comenzar a utilizar Link Inspector.

## Funcionamiento
Para comprobar los enlaces de un documento simplemente se debe arrastrar y soltar (*drag and drop*) un archivo a la zona habilitada. También se puede hacer clic en la misma para utilizar el selector de archivos del sistema operativo.

Tras el análisis de los enlaces verás una lista de registros con las siguiente información:
- **Color y status code**: dentro de este indicador pueden aparecer varias opciones: 
    - El color **verde** significa que el enlace es válido y funcional.
    - El color **amarillo** indica que el enlace ha acabado devolviendo un status code correcto, pero para ello ha sido necesario algún tipo de redirección. Esta redirección puede ser por motivos del hostname (*www*), esquema o protocolo (*http*, *https*), redirecciones por logueo, etc. En estos casos se recomienda verificar la URL manualmente y en caso necesario modificarse por la definitiva.
    - El color **rojo** significa que el stutus code recibido es incorrecto y no se ha podido verificar el enlace. Es posible que el enlace realmente funcione, pero que Link Inspector no haya podido comprobarlo por carecer de permisos de autentificación en la petición, esperar un método distinto, etc. Es importante fijarse en el número para interpretar correctamente esta información (en esta [página](https://developer.mozilla.org/es/docs/Web/HTTP/Status) se detallan los significados de cada uno). 
    - El color **gris** significa que el enlace no puede comprobarse porque no se trata de un protocolo de transferencia de hipertexto (*http*). Estos casos deben revisarse para confirmar si el protocolo es el deseado o simplemente se ha tratado de un descuido al incorporar el enlace. 
- **Texto**: el texto hipervinculado por el enlace. Puede estar vacío en caso de que el enlace se haya aplicado a otro tipo de objetos (imágenes, etc.) 
- **URL**: dirección a la que apunta el enlace.

Por otra parte, la utilidad de **Exportar** no solo permite la consulta o la automatización de acciones, también facilita información más detallada en caso de errores, redirecciones, etc.

## Interfaz de líneas de comandos (CLI - Command Line Interface)
Es posible el uso comandos en caso de de necesitar la automatización de la revisión de múltiples documentos (o simplemente porque se prefiera). 

La interfaz por línea de comandos puede generar los informes en dos tipos de archivos para datos estructurados: CSV y JSON.

Para usar esta opción debe proporcionarse la ruta de un archivo a través de **--input** (**-i**) y facilitar el destino del archivo o archivos que se desean obtener: **--csv** (**-csv**) o **--json** (**-j**).

Ejemplo de uso:
```bash
./link_inspector --input /home/usuario/Descargas/MiDocumento.txt --csv /home/usuario/Descargas/MiDocumentoEnlaces.csv -j /home/usuario/Descargas/MiDocumentoEnlaces.json
```

Resumen:
- --input o -i (obligatorio)
- --csv o -c (opcional)
- --json o -j (opcional)

**Siempre debe proporcionarse, como mínimo, uno de los campos opcionales** 

## Información para desarrolladores y colaboradores
Este proyecto ha sido construido utilizando las siguientes tecnologías:
- **Frontend:** Astro con componentes diseñados en React.
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

## Capturas
![Alt text](https://imgur.com/ffVsbJP.png "link inspector")
![Alt text](https://imgur.com/hwtrKgK.png "link inspector")
![Alt text](https://imgur.com/wURsIb3.png "link inspector")
![Alt text](https://imgur.com/LnXbVuY.png "link inspector")