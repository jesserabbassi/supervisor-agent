# Ninety Gaming House — Supervisor Desktop

GPUI desktop application for venue supervisors. The UI follows the supplied dark navy, cyan, and orange design and keeps the supplied architecture boundary:

| Layer | Files | Responsibility |
| --- | --- | --- |
| GPUI presentation | `src/views.rs`, `src/views/*.rs` | App shell and one render module per screen |
| View models | `src/viewmodels.rs` | Feature view models own selection, filters, and loaded data |
| Services | `src/services.rs` | Reservation, customer, station, wallet, session, monitoring, and remote control services |
| Infrastructure | `src/infrastructure.rs` | API client, SignalR client, local storage interface |

## Run

```sh
cargo run
```

On macOS, the project enables GPUI runtime Metal shaders and the `font-kit` text backend. The application currently starts in **Demo mode** with representative local data. No backend request or remote machine command is sent.

The app bundles IBM Plex Sans and registers it with GPUI at startup. The font license is in `assets/fonts/ibm-plex-sans/license.txt`.

## Integration handoff

The backend team should provide the API base URL, authentication mechanism, route definitions, request and response schemas, SignalR hub URL, event names and payloads, and command acknowledgement/error semantics. Connect those contracts inside `infrastructure.rs` and `services.rs`; the presentation layer should continue to call view models and services only.

The intended feature path is `View → feature ViewModel → feature Service → ApiClient / SignalRClient`. `SupervisorApp` owns the feature view models and routes pages. The current service implementations read from demo fixtures; they do not yet call a working API or subscribe to a working SignalR hub. The transport clients are placeholders, and request/response types have not been agreed with the backend team. This is a structural match to the diagram, not an integration-ready implementation.

Station lock/unlock and session controls update the local demo state and show an integration notice. Remaining action buttons are visual placeholders until their backend contracts and input forms are available. Figures and dates are representative design data, not live venue metrics.
