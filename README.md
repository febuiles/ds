ds
==
`ds` is a service to control system daemons. It provides a short and concise
  interface to `launctl(1)` (OS X), `init(8)` and `upstart(7)` (standard Linux and Debian-based
  distros).

Usage
-----

    ds start name
    ds stop name
    ds restart name
    ds info
    ds info name
    ds enabled
    ds disabled

Examples:

    ds start mysql # starts the service
    ds stop mysql # stops the service
    ds restart postgresql # restarts the service
    ds info # returns status information for all the services
    ds info sshd # returns detailed information for a specific service
    ds enabled # returns a list of the enabled (not necessarily running!) services
    ds disabled # returns a list of the disabled services

Advanced Usage
--------------

    ds reset name
    ds debug name
    ds kill name [signal]

TODO
----
* Integrate the concept of domains (maybe).
* Aliases for debugging support (`attach`/`examine` and `--session`).
* Capacity to manipulate logging facilities and options.
* Add support for adding environment variables on `start`.

Please send pull requests or open a Feature Request on the
[tracker](http://github.com/febuiles/ds/issues) if you want to see these integrated.
