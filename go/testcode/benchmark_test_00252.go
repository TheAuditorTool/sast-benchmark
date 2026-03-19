package testcode

import (
	"encoding/binary"
	"net/http"
)

type sensorReading struct {
	DeviceID    uint32
	Temperature float64
	Humidity    float64
	Timestamp   int64
}

func BenchmarkTest00252(w http.ResponseWriter, r *http.Request) {
	var reading sensorReading
	err := binary.Read(r.Body, binary.LittleEndian, &reading)
	if err != nil {
		http.Error(w, "binary decode error", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec("INSERT INTO sensor_data (device_id, temperature, humidity, timestamp) VALUES (?, ?, ?, ?)",
		reading.DeviceID, reading.Temperature, reading.Humidity, reading.Timestamp)
	if err != nil {
		http.Error(w, "storage error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"device_id":   reading.DeviceID,
		"temperature": reading.Temperature,
		"humidity":    reading.Humidity,
	})
}
