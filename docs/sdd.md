![Aetheric Banner](https://github.com/aetheric-oss/.github/raw/main/assets/doc-banner.png)

# Software Design Document (SDD) - `svc-FIXME`

## :telescope: Overview

This document details the software implementation of FIXME.

This service is responsible for FIXME

### Metadata

| Attribute     | Description                                                                    |
| ------------- |--------------------------------------------------------------------------------|
| Maintainer(s) | [@aetheric-oss/dev-realm](https://github.com/orgs/aetheric-oss/teams/dev-realm)|
| Stuckee       |                                                                                |
| Status        | Draft                                                                          |

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/aetheric-oss/se-services/blob/develop/docs/conops.md) | Overview of Aetheric microservices.
[High-Level Interface Control Document (ICD)](https://github.com/aetheric-oss/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Aetheric microservices.
[Requirements - `svc-FIXME`](FIXME - generate a link specifically for this module's view in NocoDB) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-FIXME`](./conops.md) | Defines the motivation and duties of this microservice.
[Interface Control Document (ICD) - `svc-FIXME`](./icd.md) | Defines the inputs and outputs of this microservice.

## :dna: Module Attributes

| Attribute       | Applies | Explanation                                                             |
| --------------- | ------- | ----------------------------------------------------------------------- |
| Safety Critical | Yes/No  | |
| Realtime        | Yes/No  | |

## :globe_with_meridians: Global Variables

**Statically Allocated Queues**

FIXME

## :gear: Logic

### Initialization

FIXME Description of activities at init

### Control Loop

FIXME Description of activities during loop As a GRPC server, this service awaits requests and executes handlers.

All handlers **require** the following environment variables to be set:
- FIXME

For detailed sequence diagrams regarding request handlers, see [REST Handlers](#mailbox-rest-handlers).
For detailed sequence diagrams regarding request handlers, see [gRPC Handlers](#speech_balloon-grpc-handlers).

### Cleanup

FIXME Description of activities at cleanup

## :mailbox: REST Handlers

FIXME flowcharts for rest handlers

## :speech_balloon: gRPC Handlers

FIXME flowcharts for gRPC handlers
