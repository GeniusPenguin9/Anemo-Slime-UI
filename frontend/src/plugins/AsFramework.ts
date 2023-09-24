import { useRoute } from 'vue-router';
import { Ref, onMounted, ref, inject, computed, reactive, onActivated } from 'vue';
import asComponents from "@/components/"
const SERVER_URL = import.meta.env.VITE_SERVER_URL;

interface ApiViewResponse {
    viewmodelId: string,
    widgetsData: any
}

export function initializeWidget(props: any) {
    const widgetsData: any = inject("widgetsData");
    const viewmodelId: any = inject("viewmodelId");
    const _data = reactive(Object.assign({}, props));
    const widgetId: string = props.widgetId;
    const data = computed(() => {
        return Object.assign(_data, widgetsData[widgetId]);
    });

    const applyWidgetsData = (newWdgetsData: any) => {
        Object.assign(widgetsData, newWdgetsData);
    }

    const postAction = async (type: string, data: any = {}) => {
        const fetchResponse = await fetch(SERVER_URL + "/api/action", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                "viewmodelId": viewmodelId.value,
                "widgetId": widgetId,
                "actionType": type,
                "data": data
            })
        });
        if (fetchResponse.status != 200 || fetchResponse.body == null)
            return Promise.reject("server unexpected response");
        const response = (await fetchResponse.json()) as ApiViewResponse;
        // const response = { viewmodelId: "123", widgetsData: { "yyyyy": { "text": "world" } } };
        applyWidgetsData(response.widgetsData);
    };

    return { props, viewmodelId, data, postAction };
}

type LoaderData = "Loading" | "Done" | Error;

export function initializeView() {
    const route = useRoute();
    const viewmodelId = ref("");
    const widgetsData = reactive({ _loader: "Loading" as LoaderData });

    const applyWidgetsData = (newWdgetsData: any) => {
        Object.assign(widgetsData, newWdgetsData);
    }

    const mounted = async () => {
        try {
            const fetchResponse = await fetch(SERVER_URL + "/api/view/" + (route.name as string), {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({})
            });
            if (fetchResponse.status != 200 || fetchResponse.body == null)
                return Promise.reject("server unexpected response");

            const response = (await fetchResponse.json()) as ApiViewResponse;
            // const response = { viewmodelId: "123", widgetsData: { "xxxxx": { "text": "hello" } } };
            viewmodelId.value = response.viewmodelId;
            applyWidgetsData(response.widgetsData);
            widgetsData._loader = "Done";
        }
        catch (e) {
            widgetsData._loader = e as Error;
        }
    };
    onMounted(mounted);

    return { viewmodelId, widgetsData };
}

export default {
    install: (app: any) => {
        app.config.globalProperties.initializeView = initializeView;
        app.config.globalProperties.initializeWidget = initializeWidget;

        for (const [name, comp] of Object.entries(asComponents)) {
            app.component(name, comp);
        }
    }
};