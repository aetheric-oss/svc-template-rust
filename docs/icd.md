# Interface Control Document (ICD) - `svc-FIXME`

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

This document defines the gRPC and REST interfaces unique to the `svc-FIXME` microservice.

Attribute | Description
--- | ---
Status | Draft

## Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-FIXME`](FIXME - generate a link specifically for this module's view in NocoDB) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-FIXME`](./conops.md) | Defines the motivation and duties of this microservice.
[Software Design Document (SDD) - `svc-FIXME`](./sdd.md) | Specifies the internal activity of this microservice.

## Frameworks

See the High-Level ICD.

## REST

FIXME - IF NO ADDITIONAL REST ENDPOINTS

This microservice implements no additional REST endpoints beyond the common REST interfaces (see High-Level ICD). (FIXME: and remove the rest of this section)

FIXME - IF ADDITIONAL REST ENDPOINTS, KEEP SECTION

See the High-Level ICD for common interfaces.


### Files

| File Location | Description |
--- | ---
`server/src/api_rest.rs` | Implements the REST endpoints.

### Authentication

See the High-Level ICD.

### Endpoints

| Endpoint | Type | Arguments | Description |
| ---- | --- | ---- | ---- |
| `/example` | GET | port_depart<br>port_arrive<br>time_range_start<br>time_range_end<br>cargo_weight_kg | This is an example REST endpoint.

## gRPC

### Files

These interfaces are defined in a protocol buffer file, `proto/grpc.proto`.

### Integrated Authentication & Encryption

See the High-Level ICD.

### gRPC Server Methods ("Services")

| Service | Description |
| ---- | ---- |
| `GetExample` | This is an example Service.<br>Replace

### gRPC Client Messages ("Requests")

| Request | Description |
| ------    | ------- |
| `ExampleQuery` | A message to illustrate an example
