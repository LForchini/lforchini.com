FROM nginx:latest

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY ./content /var/www/static

CMD ["nginx", "-g", "daemon off;"]