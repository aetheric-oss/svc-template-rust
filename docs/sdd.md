# \<MODULE NAME\> - Software Design Document (SDD)

<center>

<img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />

</center>

## Overview

This document details the software implementation of FIXME.

This process is responsible for FIXME

Attribute | Description
--- | ---
Status | Draft

## Related Documents

Document | Description
--- | ---
[Concept of Operations (CONOPS) (FIXME)](./FIXME) | Overview of the scope and duties of this module.
[Requirements & User Stories (FIXME)](./FIXME) | Requirements and user stories for this module.

## Location

FIXME

## Module Attributes

Attribute | Applies | Explanation
--- | --- | ---
Safety Critical | ? | 
Realtime | ? |

## Global Variables

**Statically Allocated Queues**

FIXME

## Logic 

### Initialization

FIXME 

### Control Loop

FIXME

### Cleanup

FIXME

## Interfaces

For a refresher of the processes in the Arrow backend, please see the [top level README.md](../README.md).

```mermaid
graph LR
    subgraph Vehicle Domain
        air((Aircraft))
    end

    subgraph Client Domain
        app(App X)
    end

    subgraph Server Domain
        module[Module A]
    end
```

FIXME description of the graph.

## Tests

FIXME

### Unit Tests

FIXME
