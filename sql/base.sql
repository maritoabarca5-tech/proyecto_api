-- Eliminar tablas si existen para evitar conflictos (CASCADE elimina automáticamente las relaciones)
DROP TABLE IF EXISTS Asistencia_Clases CASCADE;
DROP TABLE IF EXISTS Clases CASCADE;
DROP TABLE IF EXISTS Miembros CASCADE;
DROP TABLE IF EXISTS Instructores CASCADE;
DROP TABLE IF EXISTS Planes CASCADE;

-- 1. Crear tabla de Planes
CREATE TABLE Planes (
    id_plan SERIAL PRIMARY KEY,
    nombre_plan VARCHAR(50) NOT NULL, -- 'Mensual', 'Anual', 'VIP'
    precio_mensual DECIMAL(10,2) NOT NULL
);

-- 2. Crear tabla de Instructores
CREATE TABLE Instructores (
    id_instructor SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    especialidad VARCHAR(50) -- 'Yoga', 'Crossfit', 'Pesas'
);

-- 3. Crear tabla de Miembros
CREATE TABLE Miembros (
    id_miembro SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    fecha_inscripcion DATE DEFAULT CURRENT_DATE,
    id_plan INT REFERENCES Planes(id_plan) ON DELETE SET NULL,
    estado_membresia BOOLEAN DEFAULT TRUE
);

-- 4. Crear tabla de Clases
CREATE TABLE Clases (
    id_clase SERIAL PRIMARY KEY,
    nombre_clase VARCHAR(100) NOT NULL,
    id_instructor INT REFERENCES Instructores(id_instructor) ON DELETE SET NULL,
    horario VARCHAR(50)
);

-- 5. Crear tabla de Asistencia
CREATE TABLE Asistencia_Clases (
    id_asistencia SERIAL PRIMARY KEY,
    id_miembro INT REFERENCES Miembros(id_miembro) ON DELETE CASCADE,
    id_clase INT REFERENCES Clases(id_clase) ON DELETE CASCADE,
    fecha_asistencia DATE DEFAULT CURRENT_DATE
);
