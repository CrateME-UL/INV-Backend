SHELL := /bin/bash

#usefull to perform quick local tests
ui-copy:
	rm -rf INV-Frontend && \
	git clone git@github.com:CrateME-UL/INV-Frontend.git && \
	cd INV-Frontend && \
	git checkout INV-142-add-place-type-filter-and-their-tag && \
	rm -rf .git .github .vscode
	echo "plugin copied: run the command again and change if needed -> git checkout <branch-name> "
	cd ../..

config:
	cd k8s && \
	kubectl create secret generic ghcr-secret --from-file=.dockerconfigjson --type=kubernetes.io/dockerconfigjson && \
	cd ..

build:
	minikube image rm inv-backend:local && \
	minikube image build -t inv-backend:local -f dockerfile.backend .

dns:
	kubectl run -i --tty dns-test --image=busybox --restart=Never -- sh
	nslookup inv-backend.default.svc.cluster.local

ingress:
	helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
	helm install my-ingress ingress-nginx/ingress-nginx
