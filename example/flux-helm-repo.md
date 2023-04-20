# Manage Helm Releases

This whole file can be "executed" with:
```
eval-md bash example/flux-helm-repo.md
```

First we have to define a source helm repository:

```yaml
---
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: podinfo
  namespace: default
spec:
  interval: 1m
  url: https://stefanprodan.github.io/podinfo
```

Now we can create a helm release:

```yaml
---
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: podinfo
  namespace: default
spec:
  interval: 5m
  chart:
    spec:
      chart: podinfo
      sourceRef:
        kind: HelmRepository
        name: podinfo
  values:
    replicaCount: 2
```

Now we can deploy them:

```bash
eval-md yaml example/flux-helm-repo.md --export | kubectl apply -f -
```
