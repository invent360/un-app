{{/*
Expand the name of the chart.
*/}}
{{- define "uno-app.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
*/}}
{{- define "uno-app.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "uno-app.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "uno-app.labels" -}}
helm.sh/chart: {{ include "uno-app.chart" . }}
{{ include "uno-app.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/part-of: uno-app
region: {{ .Values.region }}
provider: {{ .Values.provider }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "uno-app.selectorLabels" -}}
app.kubernetes.io/name: {{ include "uno-app.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app: uno-app
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "uno-app.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "uno-app.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Cloudflared labels
*/}}
{{- define "cloudflared.labels" -}}
helm.sh/chart: {{ include "uno-app.chart" . }}
app.kubernetes.io/name: cloudflared
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/part-of: uno-app
app: cloudflared
{{- end }}

{{/*
Cloudflared selector labels
*/}}
{{- define "cloudflared.selectorLabels" -}}
app.kubernetes.io/name: cloudflared
app.kubernetes.io/instance: {{ .Release.Name }}
app: cloudflared
{{- end }}

{{/*
Monitoring labels
*/}}
{{- define "monitoring.labels" -}}
helm.sh/chart: {{ include "uno-app.chart" . }}
app.kubernetes.io/name: grafana-agent
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
app.kubernetes.io/part-of: uno-app
app: grafana-agent
{{- end }}

{{/*
Monitoring selector labels
*/}}
{{- define "monitoring.selectorLabels" -}}
app.kubernetes.io/name: grafana-agent
app.kubernetes.io/instance: {{ .Release.Name }}
app: grafana-agent
{{- end }}
