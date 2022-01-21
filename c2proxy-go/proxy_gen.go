// Code generated by github.com/filecoin-project/venus-market/tools/api_gen. DO NOT EDIT.

package c2proxy_go

import (
	"golang.org/x/xerrors"
)

type C2ProxyStruct struct {
	C2ProxyWorkerStruct

	C2ProxyClientStruct

	Internal struct {
	}
}

type C2ProxyStub struct {
	C2ProxyWorkerStub

	C2ProxyClientStub
}

type C2ProxyClientStruct struct {
	Internal struct {
		GetTask func(p0 int64) (Task, error) ``

		SubmitTask func(p0 []byte, p1 string, p2 [32]byte, p3 int64) (int64, error) ``
	}
}

type C2ProxyClientStub struct {
}

type C2ProxyWorkerStruct struct {
	Internal struct {
		FetchTodo func(p0 string) (Task, error) ``

		RecordError func(p0 string, p1 int64, p2 string) (bool, error) ``

		RecordProof func(p0 string, p1 int64, p2 string) (bool, error) ``
	}
}

type C2ProxyWorkerStub struct {
}

func (s *C2ProxyClientStruct) GetTask(p0 int64) (Task, error) {
	return s.Internal.GetTask(p0)
}

func (s *C2ProxyClientStub) GetTask(p0 int64) (Task, error) {
	return *new(Task), xerrors.New("method not supported")
}

func (s *C2ProxyClientStruct) SubmitTask(p0 []byte, p1 string, p2 [32]byte, p3 int64) (int64, error) {
	return s.Internal.SubmitTask(p0, p1, p2, p3)
}

func (s *C2ProxyClientStub) SubmitTask(p0 []byte, p1 string, p2 []byte, p3 int64) (int64, error) {
	return 0, xerrors.New("method not supported")
}

func (s *C2ProxyWorkerStruct) FetchTodo(p0 string) (Task, error) {
	return s.Internal.FetchTodo(p0)
}

func (s *C2ProxyWorkerStub) FetchTodo(p0 string) (Task, error) {
	return *new(Task), xerrors.New("method not supported")
}

func (s *C2ProxyWorkerStruct) RecordError(p0 string, p1 int64, p2 string) (bool, error) {
	return s.Internal.RecordError(p0, p1, p2)
}

func (s *C2ProxyWorkerStub) RecordError(p0 string, p1 int64, p2 string) (bool, error) {
	return false, xerrors.New("method not supported")
}

func (s *C2ProxyWorkerStruct) RecordProof(p0 string, p1 int64, p2 string) (bool, error) {
	return s.Internal.RecordProof(p0, p1, p2)
}

func (s *C2ProxyWorkerStub) RecordProof(p0 string, p1 int64, p2 string) (bool, error) {
	return false, xerrors.New("method not supported")
}

var _ C2Proxy = new(C2ProxyStruct)
var _ C2ProxyClient = new(C2ProxyClientStruct)
var _ C2ProxyWorker = new(C2ProxyWorkerStruct)
