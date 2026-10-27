export default function createProxy<T extends object>(target: T, handler: ProxyHandler<T>): T {
	return new Proxy(target, {
		...handler,
		get(target, p, receiver) {
			// TODO: no easy way of allowing handler.get to return something bound
			// to a separate thisArg
			const orig = (handler.get ?? Reflect.get)(target, p, receiver);
			return typeof orig === "function" && p !== "prototype" ? orig.bind(target) : orig;
		}
	});
}
