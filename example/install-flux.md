# Install Flux on a Cluster

<details>
<summary>First some preparation for variables</summary>

```bash
repo=""
branch="main"
cluster_path="./clusters/management"

help() {
  echo " --help            This help message"
  echo " --repo <repo>     Git repository. (example: git@github.com/org/repo)"
  echo " --branch <branch> Git branch. [default: ${branch}]"
  echo " --path <path>     Path to the cluster. [default: ${cluster_path}]"
}

while [[ $# -gt 0 ]]; do
  case "${1}" in
    --repo)
      repo="${2}"
      shift
      ;;
    --branch)
      branch="${2}"
      shift
      ;;
    --path)
      cluster_path="${2}"
      shift
      ;;
    --help)
      help
      exit 0
      ;;
  esac
  shift
done

if [ "${repo}" = "" ]; then
  echo ' !! git repository is not defined.'
  help
  exit 1
fi
```

</details>

1. Install Flux:

```bash
curl -s https://fluxcd.io/install.sh | sudo bash
```

2. Bootstrap Flux on the cluster:

```bash
flux bootstrap git \
  --url=ssh://${repo} \
  --branch=${branch} \
  --path=${cluster_path}
```
