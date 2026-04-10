# Design Specification

**Version:** 1.0.0
**Design Mode:** GUI (Web SPA)
**Aesthetic Theme:** "Forge Dark" (Industrial Minimalist)

## 1. Aesthetic Direction: "Forge Dark"
- **Purpose**: A hyper-functional, dense data management interface that feels technical but highly crafted.
- **Tone**: Industrial / Minimalist. Cold metal and warm sparks.
- **Differentiation Anchor**: The tension between the icy slate background and the warm amber/copper interactive elements, paired with the structural use of Space Grotesk (headings) and JetBrains Mono (data). Avoids the ubiquitous "SaaS Blue" template look.

### 1.1 Typography
- **Headings & UX Text**: `Space Grotesk` (expressive, geometric, technical)
- **Data, Numbers, Identifiers**: `JetBrains Mono` (rigid, utilitarian, alignment-safe)

### 1.2 Color Palette (CSS Variables)
- **Background (Base)**: Dark slate-charcoal (`#111827` or similar). Very subtle 1-2% noise grain overlay.
- **Surface (Elevated)**: Slightly lighter slate (`#1F2937`) with solid 1px borders (`#374151`).
- **Accent (Primary)**: Warm amber/copper (`#D97706`). Used sparsely for active states, primary buttons, and the `B` monogram.
- **Accent (Negative)**: Muted red-slate for destructive/revoked states.

### 1.3 Motion & Spatial Composition
- **Composition**: Dense, grid-aligned, crisp square corners (no border radius). Functional negative space, but not airy. Table rows have alternating micro-stripes (1-2% opacity variance) for readability.
- **Motion**: Purposefully sparse. A single 200ms `translateY(8px) -> 0` + `opacity: 0 -> 1` entrance sequence for the main content area gracefully smoothing hard route changes. Fast 100ms color transitions on borders (slate -> amber) for focus states.

---

## 2. Screen Inventory

| Screen | Route | Mockup Reference |
|---|---|---|
| **Login** | `/admin/login` | `mockups/forge_dark_login_1775796869645.png` |
| **Dashboard** | `/admin` | `mockups/forge_dark_dashboard_1775796884178.png` |
| **Collection List** | `/admin/collection/:entity` | `mockups/forge_dark_collection_1775796895853.png` |
| **Form Editor** | `/admin/collection/:entity/:id` | `mockups/forge_dark_form_editor_1775796925111.png` |
| **API Keys** | `/admin/api-keys` | `mockups/forge_dark_api_keys_1775796936519.png` |

*(Mockup images are stored in `design/mockups/`)*

---

## 3. Component Inventory

### Navigation
- **Left Sidebar**: Monogram `B` top left. Menu items with standard line-icons. Active state is NOT a full background highlight — it is indicated solely by a 3px vertical amber strip on the left edge.
- **Top Header**: Breadcrumbs (`Dashboard > Collections > Blog Posts`), User Avatar, text Logout button.

### Form Elements
- **Inputs & Textareas**: Dark inset backgrounds (`#111827`), square corners, thin slate borders. On focus: border color transitions to amber.
- **Selectors**: Dropdown menus match input styling.
- **Primary Action (Save, Sign In)**: Solid amber fill, dark charcoal text, square corners.
- **Secondary Action (Cancel)**: Transparent fill, thin slate outline, muted slate text.

### Data Elements
- **Table**: Dense. `JetBrains Mono` for all data rows. Alternating micro-stripe row backgrounds. 1px bottom border on rows.
- **Badges / Status Pills**: Solid capsules. Amber for positive (Published/Active), red-slate for negative (Draft/Revoked).
- **Stat Cards**: Thin 1px slate borders, values in `JetBrains Mono`.

---

## 4. Interaction Flows

1. **Auth Flow**: User lands on `/admin/login` → Enters credentials → Validated via server session → Redirected to `/admin` dashboard.
2. **Entity Management**:
   - Navigate to `/admin/collection/posts` (Collection List).
   - Click "Create New" or click an existing row.
   - Redirect to `/admin/collection/posts/new` or `/admin/collection/posts/:id` (Form Editor).
   - Form fields are dynamically rendered based on the entity schema fetched from API.
   - Click "Save" → POST/PUT to API → Redirect back to list view.
3. **API Key Management**:
   - Navigate to `/admin/api-keys`.
   - Table shows existing tokens.
   - Click "Create New Key" → Generates key → Modal displays raw key once (cannot be seen again).
   - Click "Revoke" → Prompts confirmation → Sends DELETE to API → Row status changes to Revoked.

---

## 5. Responsive / Resize Behavior

- **Desktop (1024px+)**: Full horizontal layout, sidebar fixed to left.
- **Tablet (768px - 1023px)**: Sidebar shrinks slightly, padding reduced, data tables allow horizontal scroll on overflow.
- **Mobile (<768px)**: Sidebar hidden behind a hamburger menu (off-canvas). Main content uses single column stacks. Data tables converted to card layout for mobile reading.
