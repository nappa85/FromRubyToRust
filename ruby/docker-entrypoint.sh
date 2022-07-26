#!/bin/bash
bundle install && rake db:migrate && rake benchmark:benchmark
