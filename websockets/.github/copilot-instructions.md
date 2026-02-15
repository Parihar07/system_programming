# Copilot Instructions for websockets Go Project

## Project Overview
This is a Go-based web application structured for modularity and clarity. It uses a layered approach:
- **cmd/web/**: Entry point for the web server, main routing logic.
- **internal/handlers/**: Business logic and HTTP handler functions.
- **html/**: Jet templates for rendering views.

## Architecture & Data Flow
- The web server starts from `cmd/web/main.go`, sets up routes in `cmd/web/routes.go`, and delegates request handling to `internal/handlers/handler.go`.
- Templates are rendered from the `html/` directory using Jet.
- All business logic is encapsulated in the `internal/handlers` package.

## Developer Workflows
- **Build**: Run `go build ./cmd/web` to build the web server.
- **Run**: Use `go run ./cmd/web` to start the server locally.
- **Dependencies**: Managed via Go modules (`go.mod`).
- **Debugging**: Standard Go debugging tools apply. Handlers are separated for easy breakpoint placement.

## Project-Specific Patterns
- **Routing**: All routes are defined in `routes.go`. Handlers are imported from `internal/handlers`.
- **Template Rendering**: Uses Jet templates. Template files are in `html/` and referenced by name.
- **Handler Structure**: Each handler function receives a context and response/request objects.

## Integration Points
- No external APIs are directly integrated; all communication is internal between server, handlers, and templates.
- Jet template engine is the main external dependency.

## Conventions
- Use `internal/` for non-public packages.
- Place all HTML templates in `html/`.
- Keep business logic out of `cmd/web/`—use handlers.
- Route definitions should be concise and reference handler functions directly.

## Examples
- To add a new route: Update `routes.go` and implement the handler in `internal/handlers/handler.go`.
- To add a new template: Place it in `html/` and reference it in the handler.

## Key Files
- `cmd/web/main.go`: Server startup
- `cmd/web/routes.go`: Route definitions
- `internal/handlers/handler.go`: Handler logic
- `html/`: Jet templates

---

Please review and suggest any missing or unclear sections for further improvement.