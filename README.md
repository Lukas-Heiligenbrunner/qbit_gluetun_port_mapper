# qbit_gluetun_port_mapper

Map qBittorrent port to Gluetun OpenVPN port.

Gluetun generates a file with the port it has opened with the vpn.
This service reads that file and updates the port via the qBittorrent API.

## Configuration

```env
GLUETUN_PATH=/tmp/forward_port
QBIT_USER=admin
QBIT_PASSWORD=<qbit_password>
QBIT_HOST=http://192.168.0.14:8081
INTERVAL=600 # update interval in seconds -> optional
LOG_LEVEL=INFO # optional
```

## License

This project is licensed under the MIT License. Feel free to contribute and modify as per the guidelines outlined in the license agreement.
