# API Drive
API Drive permite realizar las siguientes operaciones con tu Google Drive:

- Listar directorios de tu Drive.

- Listar archivos de un directorio específico.

- Subir archivos PDF.

- Descargar archivos PDF.

## Configuración del Proyecto Local
### Clonar el Repositorio
- git clone <URL_DEL_REPOSITORIO>
- cd <NOMBRE_DEL_REPOSITORIO>

## Instalación

Puedes ejecutar el proyecto usando **Rust** o **Docker**.

### Opción 1: Ejecutar con Cargo (requiere Rust)

Asegúrate de tener **Rust** instalado en tu sistema. Si no lo tienes, puedes instalarlo siguiendo las instrucciones en [Rust Official Website](https://www.rust-lang.org/).

### Para ejecutar la API con Cargo:
cargo run

### Para ejecutar la API con Docker:
**Construir la imagen Docker:**
docker build -t api_drive . para construir la imagen

**Ejecutar el contenedor Docker:**
docker run -d -p 8080:8080 --env-file .env api_drive


## Configuración del Archivo .env
Antes de ejecutar el proyecto, debes configurar un archivo .env en la raíz del proyecto con tus credenciales de la API de Google. Este archivo debe contener las siguientes variables:

CLIENT_ID=<tu_client_id>

CLIENT_SECRET=<tu_client_secret>

REDIRECT_URI=http://127.0.0.1:8080/auth/callback

AUTH_URI=https://accounts.google.com/o/oauth2/auth

TOKEN_URI=https://oauth2.googleapis.com/token

REVOKE_URI=https://oauth2.googleapis.com/revoke?token={}

SCOPE=https://www.googleapis.com/auth/drive

SERV_ADDRS=127.0.0.1:8080

GOOGLE_DRIVE_API_BASE_URL=https://www.googleapis.com/drive/v3/files

GOOGLE_DRIVE_UPLOAD_URL=https://www.googleapis.com/upload/drive/v3/files

Solo necesitas configurar CLIENT_ID y CLIENT_SECRET con tus credenciales de la API de Google, las demas variables tienen valor por default en caso de no especificarse.


## Uso de la API
### Autenticación mediante OAuth
Para autenticarte y obtener el token de acceso, sigue estos pasos:

1.- Accede a la ruta de autenticación:

2.- Ve a la ruta http://127.0.0.1:8080/auth Esto generará una URL que aparecerá en la respuesta.

### Autenticación en el navegador:

Copia y pega la URL proporcionada en tu navegador. Te pedirá que inicies sesión con tu cuenta de Google Drive.

### Redirección a Callback:

Al autenticarte, serás redirigido a la ruta http://127.0.0.1:8080/auth/callback, donde obtendrás el token de acceso.

### Uso del Token de Acceso
Este token de acceso es necesario para realizar peticiones autenticadas a la API. Puedes usar este token de dos maneras:

- En Postman o cualquier cliente http: Configura el apartado de autenticación en Postman seleccionando "Bearer Token" e introduce el token de acceso.

- En Swagger: Accede a la documentación Swagger en http://127.0.0.1:8080/swagger-ui/ , donde podrás ingresar el token en la sección de autenticación para desbloquear el acceso a las rutas protegidas.

## Documentación Swagger
La API incluye documentación Swagger para facilitar el uso de las rutas. Para acceder a la documentación:

Accede a la URL http://127.0.0.1:8080/swagger-ui/

Sigue las instrucciones en la interfaz para interactuar con las diferentes rutas de la API.

## Resumen de Rutas
- GET /drive/list-folders: Lista todos los directorios en tu Google Drive.

- GET /drive/files?folder_id=<ID_DEL_FOLDER>: Lista los archivos dentro de un directorio específico.

- POST /drive/files?folder_id=<ID_DEL_FOLDER>: Sube un archivo PDF a un directorio específico.

- GET /drive/files/{file_id}: Descarga un archivo PDF desde tu Google Drive usando su ID.