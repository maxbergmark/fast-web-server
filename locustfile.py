import time
from locust import HttpUser, task, between

class QuickstartUser(HttpUser):
    # wait_time = between(1, 5)

    @task
    def test(self):
        self.client.get("/test")

