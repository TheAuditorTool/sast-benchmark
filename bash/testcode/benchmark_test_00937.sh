#!/bin/bash
get_weather_data() {
    curl -s "https://api.weather.internal/current"
}
