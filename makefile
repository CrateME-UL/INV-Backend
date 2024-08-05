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