# ha-poweroff
This is a simple web server that allows me to integrate my current OS into Home Assistant.

It has 4 endpoints:

## GET /current
Returns the current OS (Linux or Windows) as plain text.

## POST /boot_os/windows
If on Windows, does nothing
If on Linux, sets the `bootnext` to Windows using `efibootmgr`, then reboots.
You can set the bootnum with the `WINDOWS_BOOTNUM` env variable (defaults to `0006`).

## POST /boot_os/linux
If on Linux, does nothing
If on Windows, just reboots, because Linux is my default boot entry.

## POST /poweroff
Powers off.

All the POST requests first send the response, then wait 1 second, then actually perform the action, to not have any missing responses.

You can change the port of the server with the `PORT` environment variable.

