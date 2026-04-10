# Phase 5: Leptos Admin SPA Architecture Blueprint

## 1. Overview
Phase 5 introduces the embedded Admin SPA for the `brom` headless CMS. To fulfill the core project objective of shipping a single, dependency-free binary with zero required frontend compilation by the end-user, the Admin UI is built as a highly dynamic, pre-compiled Leptos Client-Side Rendered (CSR) application.

## 2. Core Architectural Decisions

### 2.1 Authentication & Authorization
**Pattern:** Bearer Tokens via LocalStorage
- `brom-server` treats the Admin SPA identically to an external API consumer for maximal decoupling.
- The `POST /admin/api/login` endpoint validates credentials and returns a Session Token in the JSON body payload: `{ "token": "<raw_token_string>" }`.
- The Leptos SPA stores this token in `localStorage`.
- All outbound SPA API requests pass through a custom `reqwest` context wrapper that automatically injects `Authorization: Bearer <token>`.
- **Tradeoff Acceptance:** This avoids all browser cookie restrictions, CORS cookie headaches, and CSRF vulnerabilities, trading them for the standard XSS mitigation responsibility (handled natively by Leptos' template escaping).

### 2.2 Global State & Schema Injection
**Pattern:** Global Context via Eager Fetch
- The SPA does not receive hardcoded macros. It loads a purely static `index.html`.
- Upon mounting, the root `<App />` component executes `create_resource` to perform a `GET /admin/api/schema` request.
- The backend serves the entire CMS data model (`EntitySchema` map) in one lightweight JSON payload.
- This payload is injected globally using `provide_context`, allowing any deeply nested component (e.g. dynamic forms, tables, and navbars) to instantly read the structure of the data it needs to render without duplicate HTTP requests.

### 2.3 Routing Strategy
**Pattern:** Deep-linking wildcard strictly scoped to `/admin` + Singular Deterministic Params
- **Backend (`brom-server`):** Axum uses `.nest("/admin/api", api_router)` *first*, and then uses a `FallbackService` exclusively attached to `.nest("/admin", spa_fallback)`. This ensures that bad API routes correctly return `404 Not Found (JSON)`, while valid browser paths return the SPA's `index.html`.
- **Frontend (`admin`):** Leptos Router manages dynamic navigation using singular, deterministic parameters: e.g., `/admin/collection/:entity`. 
- **Advantage:** By routing to `:entity` (e.g., `post`, `blog_category`), we completely bypass the bug-prone complexity of Rust macro pluralization (e.g., "categories") while remaining perfectly determinable by the JSON schema key.

### 2.4 Component Design
**Pattern:** Dynamic Metadata Rendering
- **Navigation:** Dynamically iterates over all top-level keys in the global schema context to generate sidebar links.
- **`<DataTable />`:** A generic grid component that reads the current `:entity` path parameter, retrieves its columns, maps `FieldType` to Leptos cell renderers (e.g. `String -> txt`, `Link<T> -> relation_link`), and handles API pagination dynamically.
- **`<EditorForm />`:** Generates dynamic `<input>`, `<select>`, and `<textarea>` nodes based on the schema's `FieldType` invariants, unifying Create and Update logic.

## 3. Implementation Checklist (Act Phase Transition)

### Crate: `brom-server` & `brom-auth`
- [ ] Add `POST /admin/api/login` returning JSON token payload.
- [ ] Add `GET /admin/api/schema` endpoint for metadata serialization.
- [ ] Integrate `rust-embed` to pack the `admin/dist` folder.
- [ ] Implement Axum `FallbackService` scoping `/admin/*` to `rust-embed` strictly avoiding `/admin/api/*`.

### Crate: `admin` (Leptos SPA)
- [ ] Initialize `Trunk.toml` with `tailwindcss` integration pipeline.
- [ ] Build global `reqwest` wrapper for Bearer token injection.
- [ ] Implement Root Data fetch (`/admin/api/schema`) and `provide_context`.
- [ ] Build Layout/Nav dynamically from context.
- [ ] Build `/admin/collection/:entity` (DataTable).
- [ ] Build `/admin/collection/:entity/:id` (EditorForm).
