# Frontend to Desktop Application Templates

Templates for turning an HTML/frontend prototype into a **self-contained local desktop application**, without requiring a cloud backend or internet connection for the application to operate.

The repository currently contains two approaches:

* **PySide6** — Python-based desktop application
* **Tauri + Rust** — Rust-based desktop application

The goal is to provide a lightweight application structure where the frontend can communicate with local backend functionality through a simple request/response interface, while application resources such as SQLite databases and configuration remain local to the application.

> **Status:** This repository is a work in progress. The PySide6 implementation is the current focus. The example/demo application is not yet complete.

---

## Motivation

This project originated from a practical problem: how to share a functional frontend prototype without requiring the recipient to set up a server, cloud resources, or external services.

Instead of deploying the prototype to a web server, the application can package its frontend, backend logic, database, and other required resources into a local desktop application.

The intended workflow is:

```text
HTML / Frontend Prototype
          │
          ▼
   Desktop Application
          │
          ├── Request Router
          │
          ├── Worker / Backend Tasks
          │
          ├── SQLite
          │
          └── Local Resources
```

This makes the prototype easier to distribute while keeping its data and supporting resources local.

---

## Design Goals

The project explores a few design principles:

### 1. Keep the frontend simple

The frontend should not need to know how the backend is implemented.

A request can be represented as a dictionary containing information such as:

```python
{
    "endpoint": "get_user",
    "params": {
        "user_id": 123
    }
}
```

The desktop application handles routing this request to the appropriate backend task.

---

### 2. Separate routing from application logic

The application framework is responsible for concerns such as:

* Request routing
* Request/response validation
* Worker execution
* SQLite setup
* Resource management
* Application configuration

The programmer can then concentrate on implementing the actual functionality of an endpoint.

Conceptually:

```text
Frontend
   │
   │ request
   ▼
Request Router
   │
   ▼
Worker Task
   │
   ├── Business Logic
   ├── Database Access
   └── Local Resources
   │
   ▼
Response
   │
   ▼
Frontend
```

---

## 3. Configuration-driven setup

The templates explore using YAML configuration to describe parts of the application structure.

For example, configuration can be used to define concepts such as:

* Request schemas
* Response schemas
* Database schemas
* Application configuration
* Endpoint definitions

The intention is to reduce repetitive setup code and make the application structure easier to understand and modify.

Example:

```yaml
endpoint:
  name: get_user

  request:
    user_id: integer

  response:
    id: integer
    name: string
```

The exact configuration format is still evolving.

---

## Implementations

### PySide6

The PySide6 implementation is currently the main development branch of the project.

The current design focuses on providing a simple request dispatcher that maps a request dictionary to the appropriate worker task.

```text
Frontend
    │
    ▼
Request Dictionary
    │
    ▼
Dispatcher
    │
    ├── Worker A
    ├── Worker B
    └── Worker C
          │
          ▼
      Application Logic
```

The intention is that adding a new endpoint should primarily involve implementing the endpoint's backend logic rather than modifying the application's routing infrastructure.

### Tauri / Rust

The repository also contains an experimental Tauri/Rust implementation exploring the same overall concept using a Rust-based desktop application architecture.

The two implementations are not intended to be identical internally. They are explorations of how the same application design can be implemented using different desktop technologies.

---

## Local Application Model

The application is designed around the idea that the desktop application owns its required resources.

Potential resources include:

```text
Application
├── Frontend assets
├── Backend code
├── Configuration
├── SQLite database
└── Other local resources
```

This allows the application to operate without depending on a remote backend for its core functionality.

The approach is particularly useful for:

* Internal prototypes
* Demonstration applications
* Offline applications
* Local tools
* Applications where cloud deployment is unnecessary for a prototype

---

## Repository Structure

```text
.
├── pyside6_template/
│   └── PySide6 implementation
│
├── tauri_template/
│   └── Tauri + Rust implementation
│
└── .github/
    └── workflows/
```

The structure may change as the templates develop.

---

## Current Status

This project is intentionally kept as an experimental/template repository rather than presented as a finished framework.

### PySide6

* [x] Basic desktop application structure
* [x] Frontend integration approach
* [x] Request/worker architecture
* [ ] Complete example application
* [ ] Finalize YAML configuration format
* [ ] Expand schema-driven setup
* [ ] Documentation and examples

### Tauri / Rust

* [x] Initial template
* [ ] Further development and refinement

The PySide6 implementation is currently the primary focus.

---

## Why This Project Exists

This repository is a **generalized and independently developed representation of an application architecture explored through practical development work**.

It does not contain proprietary code, data, or implementation details from previous projects.

The purpose of the repository is to demonstrate the underlying engineering ideas:

* Desktop application architecture
* Separation of frontend and backend concerns
* Local data management
* Configuration-driven application setup
* Request routing and worker execution
* Cross-technology architectural exploration

---

## Future Direction

The project may eventually evolve toward a more generic template where a developer can define application components through configuration and implement only the application-specific backend logic.

Possible future work includes:

* More complete YAML schemas
* Automatic SQLite/database initialization
* Request and response validation
* Endpoint registration
* Worker lifecycle management
* Packaging into standalone desktop executables
* A complete example application

For now, the repository serves primarily as an exploration of these ideas and as a reusable starting point for local desktop applications.
