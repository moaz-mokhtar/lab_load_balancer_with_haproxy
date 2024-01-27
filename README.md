# Load Balancer with HAProxy

- This is to test load balancer using HAProxy with backend servers using Rust and Actix-Web.
- 2 servers are created `Server 1` and `Server 2` by Actix-Web.
- Haproxy is used as load balancer with algorithm round-robin, so that means each request will be sent to next server as below example:
```
    curl http://localhost:7500/ => Server 1
    curl http://localhost:7500/ => Server 2
    curl http://localhost:7500/ => Server 1
    curl http://localhost:7500/ => Server 2
    ...
```


## Testing Load Balancer

```shell
❯ curl http://localhost:7500/
	 - Status: Healthy - Server 1 - Time: 2024-01-27 14:00:20.354658660 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 2 - Time: 2024-01-27 14:00:22.423591601 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 1 - Time: 2024-01-27 14:00:23.480433109 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 2 - Time: 2024-01-27 14:00:24.839409950 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 1 - Time: 2024-01-27 14:00:25.662647098 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 2 - Time: 2024-01-27 14:00:25.856315675 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 1 - Time: 2024-01-27 14:00:26.035032038 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 2 - Time: 2024-01-27 14:00:26.232938583 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 1 - Time: 2024-01-27 14:00:26.407342828 +02:00

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 2 - Time: 2024-01-27 14:00:26.612424355 +02:00^[[A

❯ curl http://localhost:7500/
	 - Status: Healthy - Server 1 - Time: 2024-01-27 14:00:27.592677557 +02:00
```

Screenshot:

![Terminals screenshot](image.png)
