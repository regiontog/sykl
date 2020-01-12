# Installing
If you are on x86-64 windows or linux you can download a prebuilt binary from [github releases](https://github.com/regiontog/sykl/releases/tag/sykl-rest-0.1.0). The binaries here are _not_ installers, but the program itself.

# Running
The web server takes no arguments and propably should be ran from the command line.

# Compiling
If you have a [rust toolchain](https://rustup.rs/) installed you can download the source from github and in the source directory run
```bash
> cargo run --bin sykl-rest
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/sykl-rest`
```

# Endpoints
- [`GET /stations`](##/stations)
- [`GET /station/:id`](##/station/:id)

# Types
```typescript
type Stations = { [key: string]: Station; }

type Station = {
    station_id: string;
    name: string;
    address: string;
    lat: number;
    lon: number;
    capacity: number;
    status: Status | null;
}

type Status = {
    is_installed: number;
    is_renting: number;
    num_bikes_available: number;
    num_docks_available: number;
    is_returning: number;
    last_reported: string;
}
```

## /stations
Returns a `Stations` object with all stations.
```bash
curl http://localhost:8080/stations
```
```json
{
    "567": {
        "station_id": "567",
        "name": "Forskningsveien",
        "address": "Forskningsveien",
        "lat": 59.9454775,
        "lon": 10.7138465,
        "capacity": 39,
        "status": {
            "is_installed": 1,
            "is_renting": 1,
            "num_bikes_available": 0,
            "num_docks_available": 39,
            "is_returning": 1,
            "last_reported": "2020-01-10T18:33:02Z"
        }
    },
    "502": {
        "station_id": "502",
        "name": "Ullevål sykehus",
        "address": "Kirkeveien / Ullevål sykehus",
        "lat": 59.9360307,
        "lon": 10.7364516,
        "capacity": 35,
        "status": {
            "is_installed": 1,
            "is_renting": 1,
            "num_bikes_available": 0,
            "num_docks_available": 35,
            "is_returning": 1,
            "last_reported": "2020-01-10T18:33:02Z"
        }
    },
    "435": {
        "station_id": "435",
        "name": "Fram-gården",
        "address": "Fram-gården",
        "lat": 59.915424,
        "lon": 10.714577,
        "capacity": 30,
        "status": {
            "is_installed": 1,
            "is_renting": 1,
            "num_bikes_available": 0,
            "num_docks_available": 30,
            "is_returning": 1,
            "last_reported": "2020-01-10T18:33:02Z"
        }
    },
    "427": {
        "station_id": "427",
        "name": "Briskeby",
        "address": "Briskeby",
        "lat": 59.920218,
        "lon": 10.717978,
        "capacity": 15,
        "status": {
            "is_installed": 1,
            "is_renting": 1,
            "num_bikes_available": 0,
            "num_docks_available": 15,
            "is_returning": 1,
            "last_reported": "2020-01-10T18:33:02Z"
        }
    }
}
```
## /station/:id
Returns a `Station` object corresponding to `:id`.

```bash
curl http://localhost:8080/station/$STATION_ID
```

```json
{
    "station_id": "590",
    "name": "Majorstua skole",
    "address": "Majorstua skole",
    "lat": 59.9294351,
    "lon": 10.7136816,
    "capacity": 40,
    "status": {
        "is_installed": 1,
        "is_renting": 1,
        "num_bikes_available": 0,
        "num_docks_available": 40,
        "is_returning": 1,
        "last_reported": "2020-01-10T18:35:33Z"
    }
}
```
