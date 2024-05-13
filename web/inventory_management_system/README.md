# Inventory Management System

## Guía de Instalación

### Pre-requisitos

1. Instalar Laravel Herd. Seguir la guía de instalación en la página web.
2. Tener un servidor SQL (recomendado MySQL).

### Guía

1. Descomprimir el archivo: Descomprimir el archivo de código y dejarlo en un directorio fácil de acceder.
2. Navegar al Directorio del Proyecto: Dirigirse al directorio recién descomprimido usando el siguiente comando:

    ```cd inventory_management_system```

3. Actualizar Dependencias con Composer: Ejecuta el siguiente comando para actualizar las dependencias del proyecto utilizando Composer:

    ```composer update```

4. Crear Archivo de Variables de Entorno: Copia el archivo de variables de entorno de ejemplo utilizando el siguiente comando:

    ```cp .env.example .env```

5. Configuración del entorno: abrir el archivo `.env` recién creado con el editor de texto de preferencia y editar las variables `DB_` con los valores correctos dependiendo del servidor utilizado. Crear una base de datos con el nombre IMS.
6. Configurar la Base de Datos: Luego, ejecuta las migraciones para crear las tablas necesarias en la base de datos:

    ```php artisan migrate```

7. Iniciar el Servidor: Para iniciar el servidor local, ejecuta el siguiente comando:

    ```php artisan serve```

8. Acceder al Sistema: Abre tu navegador web y navega a la siguiente URL http://127.0.0.1:8000/
