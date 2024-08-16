#!/usr/bin/env bash

function calculate_bins_for() {
	if [ -z "$1" ]; then
		read -pr "Please provide a floating-point range from a RangeIndex (f32): " range
	else
		range="$1"
	fi

	result="["

	for index in $(seq 1 250); do
		value="$(echo "${index} * (${range} / 250)" | bc -l)"
		value="$(printf "%.10g" "${value}")"
		if [[ "${value}" =~ ^[0-9]+$ ]]; then
			value+=".0"
		fi

		if [ "${index}" -eq 250 ]; then
			result+="${value}"
		else
			result+="${value}, "
		fi
	done

	result+="]"
	echo "${result}"
}

function calculate_bins() {
	if [ "$1" == "all" ]; then
		for range in "0.125" "0.25" "0.5" "0.75" "1" "2" "3" "4" "5" "6"; do
			echo "// Range: ${range}m"
			calculate_bins_for "${range}"
			echo
		done
		return
	fi

	calculate_bins_for "$1"
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
	calculate_bins "$@"
fi
