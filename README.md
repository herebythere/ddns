# Ddns

Send ip address updates to DNS services.

## How to use

The following sections describe how to install and run `ddns`.

### Install

Run the following shell commands:

```sh
git clone https://github.com/w-lfpup/ddns
cargo install --path ddns
```

All services are directly related to features.

All features (and therefore all services) are included by default.

#### Minimal installs

Minimal installations should use the `--features` flag.

For example, the following script will install `ddns` but only support dyndns2:

```sh
cargo install --path ddns --features dyndns2
```

### Configuration

The `ddns` application requires a valid JSON configuration to run.

An example configuration example can be found at
`./ddns.example.json`

```JSON
{
	"results_filepath": "./path_to_results.json",
	"ip_services": [
		"https://checkip.amazonaws.com/",
		"https://api.ipify.org"
	]
}
```

The `results_filepath` and `ip_services` properties are required. 

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of `services` with a `url` and its `response_type`.

### Run ddns

The `ddns` application accepts one argument defining a path to a configuration file.

```
ddns <path_to_json_config>
```

The results of the `ddns` will be written to the `results_filepath` property of the `config`.

Paths can be absolute or relative to the configuration file.

## Available services

The `ddns` application provides support for the following `services`:

- [dyndns2](#dyndns2)
- [cloudflare](#cloudflare)

### Dyndns2

Use the following schema to add `dyndns2` domains to the `config`.

```JSON
{
	"domain_services": {
		"dyndns2": [{
			"service_uri": "string",
			"hostname": "string",
			"username": "string",
			"password": "string"
		}]
	}
}
```

Standard dyndns2 `path` and `parameters` will be appended to the authority of the `service_uri` property.

So `https://example-ddns-service.com` will become:

```
https://example-ddns-service.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4
```

### Cloudflare

Use the following schema to add `cloudflare` domains to the `config`.


```JSON
{
	"domain_services": {
		"cloudflare": [{
			"name": "yourdomain.com",
			"email": "string",
			"zone_id": "string",
			"dns_record_id": "string",
			"api_token": "string",
			"type": "string, record type ie: A",
			"proxied": "bool | null",
			"comment": "string | null",
			"tags": "[]string | null",
			"ttl": "number | null",
		}]
	}
}
```

### Secrets

Any password or api key can be derived from an environment variable.

To pull an environment variable, use the prefix `ENV:` followed by the
variable name as demonstrated by the following json snippet.

```json
{
	"password": "ENV:MY_SECRET_PASSWORD",
	"api_token": "ENV:MY_API_TOKEN"
}
```

## Licence

BSD 3-Clause License
