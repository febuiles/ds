ds (daemon services)
==
`ds` is a wrapper for some Unix daemon control facilities. It provides a short and concise
  interface to `launctl(1)` (OS X), `init(8)` and `upstart(7)` (standard Linux and Debian-based
  distros).

Usage
-----

    ds start (name)
    ds stop (name)
    ds restart (name)
    ds info
    ds info (name)
    ds enabled
    ds disabled

Examples:

    $ ds start mysql            # starts the service
    * Starting MySQL. [OK]

    $ ds stop mysql             # stops the service
    * Stopping MySQL. [OK]

    $ ds restart postgresql     # restarts the service
    * Restarting MySQL. [OK]

    $ ds info                   # returns status information for all the services
    # OS X
    * 5406 daemon (com.apple.CoreAuthentication.daemon) [Running]
    * 1153 ssh-agent (org.openbsd.ssh-agent) [Running]
    * [-] daemon (com.apple.CoreAuthentication.daemon) [Unknown] (status: -43)

    # Linux
    * 3417 atd (/etc/init.d/atd) [Running]
    * 3400 tty3 (/etc/init/tty3.conf) [Running]
    * [-] plymouth (/etc/init/plymouth) [Stopped]
    * [-] dmesg (/etc/init/dmesg) [Disabled]

    $ ds info sshd                   # returns detailed information for a specific service
    Real Name: org.openbsd.ssh-agent
    File: ~/Library/LaunchAgents/org.openbsd.ssh-agent
    Status: Running
    Code: 0

    $ ds enabled                   # returns a list of the enabled (not necessarily running!) services
    * 3417 atd (/etc/init.d/atd)
    * 3400 tty3 (/etc/init/tty3.conf)

    $ ds disabled                   # returns a list of the disabled services
    * [-] dmesg (/etc/init/dmesg)

Advanced Usage
--------------

    ds reset name
    ds kill name [signal]

Examples:

    $ ds reset mariadb              # Reload the source file and restarts the service. Useful for debugging scripts.

    # All of these are equivalent, the signal can be either a name or a number.
    $ ds kill apache2 9
    $ ds kill apache2 KILL
    $ ds kill apache2 kill
    $ ds kill apache2 SIGKILL



TODO
----
* Integrate the concept of users/domains (maybe).
* Aliases for debugging support (`attach`/`examine` and `--session`).
* Capacity to manipulate logging facilities and options.
* Support for adding environment variables on `start`.

Please send pull requests or open a Feature Request on the
[tracker](http://github.com/febuiles/ds/issues) if you want to see these integrated.
